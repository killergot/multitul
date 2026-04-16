use std::collections::HashMap;
use log::info;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use super::commit::Commit;

use crate::utils::git::git_error::GitError;
use crate::utils::git::git_ref::GitRef;
use crate::utils::git::hash::Hash;
use crate::utils::git::ref_name::RefName;
use crate::utils::git::ref_target::RefTarget;
use crate::utils::git::repository::Repository;
use flate2::read::ZlibDecoder;
use crate::utils::git::consts::{FAN_OUT_OFFSET_V2, FAN_OUT_SIZE, HASH_LEN_SHA1};

pub struct GitStorage {
    main_path: PathBuf,
    verbose: bool,
    pack_files: Vec<PackFiles>,
}

struct PackFiles {
    hash: String,
    idx: PackFileType,
    pack: PackFileType,
    rev: Option<PackFileType>,
}

impl PackFiles {
    pub fn get_fanout_as_bytes(&self) -> Option<&[u8]>{
        self.idx.get_fanout_as_bytes()
    }
}


#[derive(Debug,Clone)]
pub enum PackFileType {
    Idx(Vec<u8>),
    Pack(Vec<u8>),
    Rev(Vec<u8>),
    Crash
}

impl PackFileType {
    fn as_slice(&self) -> Option<&[u8]> {
        match self {
            PackFileType::Idx(v)
            | PackFileType::Pack(v)
            | PackFileType::Rev(v) => Some(v),
            PackFileType::Crash => None,
        }
    }

    fn get_fanout_as_bytes(&self) -> Option<&[u8]> {
        match self {
            PackFileType::Idx(v) =>
                {
                    let idx = &v[FAN_OUT_OFFSET_V2..];
                    Some(&idx[..FAN_OUT_SIZE])
                },
            _ => None
        }
    }
}



impl GitStorage {
    pub fn new<P: AsRef<Path>>(main_path: P) -> Self {
        GitStorage {
            main_path: main_path.as_ref().to_path_buf(),
            verbose: false,
            pack_files: Vec::new(),
        }
    }

    pub fn read_head(&self) -> Result<String, GitError> {
        let content = fs::read_to_string(self.main_path.join("HEAD"))?;
        Ok(content)
    }

    pub fn read_commit_by_hash(&self, hash: &str) -> Result<String, GitError> {
        if hash.len() < HASH_LEN_SHA1 {
            return Err(GitError::InvalidObject("Hash is too short".to_string()));
        }

        let dir_commit = self.main_path.join("objects").join(&hash[0..2]);

        let compressed = fs::read(&dir_commit.join(&hash[2..]))?;

        let mut decoder = ZlibDecoder::new(&compressed[..]);
        let mut decoded = String::new();
        decoder.read_to_string(&mut decoded)?;
        Ok(decoded)
    }

    pub fn read_commit_by_ref(&self, refs: &Path) -> Result<(String, String), GitError> {
        let commit_uid = fs::read_to_string(&refs)?;
        let commit_uid = commit_uid.trim();

        info!(target: "git", "Reading commit {}", &commit_uid);

        let raw_commit = self.read_commit_by_hash(commit_uid)?;
        Ok((commit_uid.to_string(), raw_commit))
    }

    pub fn read_ref(&self, refname: &Path) -> Result<String, GitError> {
        let commit_uid = fs::read_to_string(refname)?;
        Ok(commit_uid.trim().to_string())
    }

    pub fn get_all_refs(&self) -> Vec<PathBuf> {
        let mut branches: Vec<PathBuf> = Vec::new();
        let local_branch = self.main_path.join("refs/heads/");
        if self.verbose {
            info!("Listing local branches:");
        }
        self._get_all_refs(local_branch.as_path(), &mut branches);
        for branch in &branches {
            if self.verbose {
                info!("{}", branch.display());
            }
        }
        branches
    }

    fn _get_all_refs<'a>(&self, subpath: &'a Path, branches: &'a mut Vec<PathBuf>) {
        if subpath.is_dir() {
            if let Ok(entries) = fs::read_dir(subpath) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        self._get_all_refs(&path, branches);
                    }
                }
            }
        } else if subpath.is_file() {
            if self.verbose {
                info!("{}", subpath.display());
            }
            branches.push(subpath.to_path_buf());
        }
    }

    pub fn _find_commit(&self, hash: Hash){
        for i in self.pack_files.iter() {
            if let Some(fanout_bytes) = i.get_fanout_as_bytes() {
                let fanout = parse_fanout(fanout_bytes);
                let count_obj = fanout[255];
                println!("Found fanout: {}", count_obj);
                let oid = parse_hash(&hash);
                let first = oid[0];
                let lo = match first {
                    0 => 0,
                    _ => fanout[(first - 1) as usize],
                };
                let hi = fanout[first as usize];
                let count = hi - lo;
                if count == 0 {
                    continue;
                }
                else{
                    println!("lo - hi = : {}", count);
                }
            }
        }
    }


    pub fn _parse_pack_files(&mut self) -> Result<String, GitError> {
       if let Ok(entries) = fs::read_dir(&self.main_path.join("objects/pack/")) {
           let mut pack_map: HashMap<String,Vec::<PackFileType>> = HashMap::new();
           for entry in entries {
               let entry = entry?.path();
               let id =  entry
                   .file_stem()
                   .and_then(|s| s.to_str())
                   .unwrap()
                   .to_string();
               if let Some(pack_file) = self._read_pack_file(&entry) {
                   pack_map
                       .entry(id)
                       .or_insert(Vec::<PackFileType>::new())
                       .push(pack_file);
               }
           }
           for (hash,files) in pack_map {
               println!("{}", hash);
               let mut idx: PackFileType = PackFileType::Crash;
               let mut pack: PackFileType = PackFileType::Crash;
               let mut rev: Option<PackFileType> = None;
               for file in files {
                   match file {
                       PackFileType::Pack(bytes) => pack = PackFileType::Pack(bytes),
                       PackFileType::Rev(bytes) => rev = Some(PackFileType::Rev(bytes)),
                       PackFileType::Idx(bytes) => idx = PackFileType::Idx(bytes),
                       _ => {}
                   }
               }
               self.pack_files.push(PackFiles{
                   hash,
                   idx: idx.clone(),
                   pack: pack.clone(),
                   rev: rev.clone(),
               })
           }
       };
        Ok("Ok".to_string())
    }

    pub fn _read_pack_file(&mut self, subpath: &Path) -> Option<PackFileType> {
        if let Ok(entry) = fs::read(subpath){
            match entry.as_slice(){
                [0xFF,0x74,0x4F,0x63,..] => Some(PackFileType::Idx(entry)),
                [0x52,0x49,0x44,0x58, ..] => Some(PackFileType::Rev(entry)),
                [0x50,0x41, 0x43, 0x4B, ..] => Some(PackFileType::Pack(entry)),
                _ => Some(PackFileType::Crash),
            }
        }
        else{None}
    }
}


fn parse_fanout(data: &[u8]) -> Vec<u32> {
    data.chunks_exact(4)
        .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
        .collect()
}

fn parse_hash(hash: &Hash) -> Vec<u8> {
    hash.0.bytes().collect()
}