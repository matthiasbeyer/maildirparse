use std::path::PathBuf;

use mailparse::parse_mail;
use mailparse::ParsedMail;

use result::Result;

pub struct Maildir {
    path: PathBuf,
    subdirs: Option<Vec<Maildir>>,
}

impl Maildir {

    pub fn from_path(p: PathBuf) -> Result<Maildir> {
        unimplemented!()
    }

    pub fn from_dir_recursive(p: PathBuf) -> Result<Maildir> {
        unimplemented!()
    }

    pub fn load_subdirs(&mut self) -> Result<&Vec<Maildir>> {
        match self.subdirs {
            Some(v) => Ok(&v),
            None => {
                unimplemented!()
            },
        }
    }

    pub fn has_subdirs(&self) -> bool {
        match self.subdirs {
            Some(s) => s.len() != 0,
            None    => false,
        }
    }

    pub fn subdirs(&self) -> Option<&Vec<Maildir>> {
        self.subdirs.as_ref()
    }

    pub fn new_mails_pathes<I: Iterator<Item = PathBuf>>(&self) -> Result<I> {
        unimplemented!()
    }

    pub fn new_mails<'a, I: Iterator<Item = ParsedMail<'a>>>(&self) -> Result<I> {
        unimplemented!()
    }

    pub fn tmp_mails_pathes<I: Iterator<Item = PathBuf>>(&self) -> Result<I> {
        unimplemented!()
    }

    pub fn tmp_mails<'a, I: Iterator<Item = ParsedMail<'a>>>(&self) -> Result<I> {
        unimplemented!()
    }

    pub fn cur_mails_pathes<I: Iterator<Item = PathBuf>>(&self) -> Result<I> {
        unimplemented!()
    }

    pub fn cur_mails<'a, I: Iterator<Item = ParsedMail<'a>>>(&self) -> Result<I> {
        unimplemented!()
    }

    fn get_mail_pathes<I: Iterator<Item = PathBuf>>(&self, subdir: &str) -> Result<I> {
        if subdir == "cur" || subdir == "new" || subdir == "tmp" {
            unimplemented!()
        } else {
            unimplemented!()
        }
    }

    fn get_mails<'a, I: Iterator<Item = ParsedMail<'a>>>(&self, subdir: &str) -> Result<I> {
        self.get_mail_pathes(subdir)
            .map(|_| unimplemented!())
    }

    pub fn has_new_mail(&self) -> Result<bool> {
        unimplemented!()
    }

    pub fn has_tmp_mail(&self) -> Result<bool> {
        unimplemented!()
    }

    pub fn has_cur_mail(&self) -> Result<bool> {
        unimplemented!()
    }

    fn has_new_mail_in(&self, subdir: &str) -> Result<bool> {
    }

    pub fn count_new_mail(&self) -> Result<usize> {
        unimplemented!()
    }

    pub fn count_tmp_mail(&self) -> Result<usize> {
        unimplemented!()
    }

    pub fn count_cur_mail(&self) -> Result<usize> {
        unimplemented!()
    }

    fn count_mail_in(&self, subdir: &str) -> Result<usize> {
        unimplemented!()
    }

}

