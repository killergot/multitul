use log::info;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::utils::git::commit::Commit;

use crate::utils::git::store::consts::{FAN_OUT_OFFSET_V2, FAN_OUT_SIZE, HASH_LEN_SHA1, HASHES_OFFSET_V2};
use crate::utils::git::git_error::GitError;
use crate::utils::git::git_ref::GitRef;
use crate::utils::git::hash::Hash;
use crate::utils::git::ref_name::RefName;
use crate::utils::git::ref_target::RefTarget;
use crate::utils::git::repository::Repository;
use crate::utils::git::store::pack::{ObjectPackType, PackFileType, PackFiles};

use flate2::read::ZlibDecoder;


pub struct GitStorage {
    main_path: PathBuf,
    verbose: bool,
    pack_files: Vec<PackFiles>,
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
        if hash.len() < HASH_LEN_SHA1 * 2 {
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

    pub fn get_commit_from_pack_file(pack: PackFileType, offset: usize) {
        if let Some(objects) = pack.get_objects_table() {
            if let Some(pack_entry) = objects.get(offset) {
                if let Some(pack_type) = parse_type_object(pack_entry){
                    println!("pack type:{:?}",pack_type)
                }
                println!("Pack entry: {:?}", pack_entry);
            }
        }
    }

    pub fn _find_commit(&self, hash: Hash) {
        for i in self.pack_files.iter() {
            println!();
            if let Some(fanout_bytes) = i.get_fanout_as_bytes() {
                let fanout = parse_fanout(fanout_bytes);
                let count_obj = fanout[255];
                println!("Found fanout: {}", count_obj);
                let oid: Vec<u8> = hex_to_bytes(&hash.0);
                let first = oid[0];
                let lo = match first {
                    0 => 0,
                    _ => fanout[(first - 1) as usize],
                };
                let hi = fanout[first as usize];
                let count = hi - lo;
                println!("Hi - Lo: {}", count);
                if count == 0 {
                    continue;
                }
                if let Some(hashes) = i.idx.get_part_of_hashes_table(lo as usize, hi as usize) {
                    println!("Found hashes: {:?}", hashes);
                    if let Some(our_index) = binary_search(&hashes, &hash.0) {
                        println!("In idx: {} found hash {}", &i.hash[..10], &hashes[our_index][..7]);
                        if let Some(offsets) = i.idx.get_offsets_table(count_obj as usize) {
                            println!("Found offset: {}", offsets[lo as usize + our_index]);
                            // тут нужна логика на старший бит: если он есть, то это индекс в large offsets таблице
                            Self::get_commit_from_pack_file(i.pack.clone(), offsets[lo as usize + our_index] as usize);
                        };
                    }
                }
            }
        }
    }

    pub fn _parse_pack_files(&mut self) -> Result<String, GitError> {
        if let Ok(entries) = fs::read_dir(&self.main_path.join("objects/pack/")) {
            let mut pack_map: HashMap<String, Vec<PackFileType>> = HashMap::new();
            for entry in entries {
                let entry = entry?.path();
                let id = entry
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
            for (hash, files) in pack_map {
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
                self.pack_files.push(PackFiles {
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
        if let Ok(entry) = fs::read(subpath) {
            match entry.as_slice() {
                [0xFF, 0x74, 0x4F, 0x63, ..] => Some(PackFileType::Idx(entry)),
                [0x52, 0x49, 0x44, 0x58, ..] => Some(PackFileType::Rev(entry)),
                [0x50, 0x41, 0x43, 0x4B, ..] => Some(PackFileType::Pack(entry)),
                _ => Some(PackFileType::Crash),
            }
        } else {
            None
        }
    }
}

fn parse_fanout(data: &[u8]) -> Vec<u32> {
    data.chunks_exact(4)
        .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
        .collect()
}

fn binary_search<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    // грокаем алгоритмы ХЕЛЛОУ
    let mut left = 0;
    let mut right = slice.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if &slice[mid] < target {
            left = mid + 1;
        } else if &slice[mid] > target {
            right = mid;
        } else {
            return Some(mid);
        }
    }

    None
}

fn hex_to_bytes(s: &str) -> Vec<u8> {
    // Сначала я юзал .bytes, что оказалось в корне не верно, оно возвращает ASCII коды, а не байты
    // Потом я юзал from_str_radix, но его я не смог нормально сам написать, так что вспомнил 1 курс
    fn val(c: u8) -> u8 {
        match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'f' => c - b'a' + 10,
            b'A'..=b'F' => c - b'A' + 10,
            _ => 0,
        }
    }

    let bytes = s.as_bytes();

    bytes
        .chunks(2)
        .map(|pair| (val(pair[0]) << 4) | val(pair[1]))
        .collect()
}


fn parse_type_object(pack_entry: &u8) -> Option<ObjectPackType>{
    println!("num type = {}", (pack_entry >> 4 ) & 7 as u8);
    match (pack_entry >> 4) & 7 as u8 {
        1 => Some(ObjectPackType::Commit),
        2 => Some(ObjectPackType::Tree),
        3 => Some(ObjectPackType::Blob),
        4 => Some(ObjectPackType::Tag),
        6 => Some(ObjectPackType::Ofs_delta),
        7 => Some(ObjectPackType::Ref_delta),
        _ => None
    }
}