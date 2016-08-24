use std::path::PathBuf;

use mailparse::parse_mail;
use mailparse::ParsedMail;
use walkdir::WalkDir;

use result::Result;
use error::MaildirError;
use error::MaildirErrorKind as MEK;

pub struct Maildir {
    path: PathBuf,
    subdirs: Option<Vec<Maildir>>,
}

impl Maildir {

    pub fn from_path(p: PathBuf) -> Result<Maildir> {
        p.read_dir()
            .map_err(Box::new)
            .map_err(|e| MEK::IOError.into_error_with_cause(e))
            .and_then(|rd| {
                let res = rd.fold(Ok((false, false, false)), |acc, elem| {
                    acc.and_then(|tpl| {
                        elem.and_then(|e| {
                            let p = e.path();
                            Ok((tpl.0 || p == PathBuf::from("./cur"),
                                tpl.1 || p == PathBuf::from("./new"),
                                tpl.2 || p == PathBuf::from("./tmp")))
                        })
                        .map_err(Box::new)
                        .map_err(|e| MEK::IOError.into_error_with_cause(e))
                    })
                });

                res.and_then(|r| match r {
                    (false, _, _) => {
                        let cause = Box::new(MEK::CurDirDoesNotExist.into_error());
                        Err(MEK::NotAMaildirError.into_error_with_cause(cause))
                    },
                    (_, false, _) => {
                        let cause = Box::new(MEK::NewDirDoesNotExist.into_error());
                        Err(MEK::NotAMaildirError.into_error_with_cause(cause))
                    },
                    (_, _, false) => {
                        let cause = Box::new(MEK::TmpDirDoesNotExist.into_error());
                        Err(MEK::NotAMaildirError.into_error_with_cause(cause))
                    },
                    (true, true, true) => {
                        Ok(Maildir {
                            path: p,
                            subdirs: None,
                        })
                    }
                })
            })
    }

    pub fn from_dir_recursive(p: PathBuf) -> Result<Maildir> {
        let mut this = try!(Maildir::from_path(p.clone()));

        let mut v = vec![];
        for entry in WalkDir::new(p) {
            if entry.file_type().is_dir() {
                match entry {
                    Ok(entry) => {
                        match Maildir::from_path(PathBuf::from(entry.path())) {
                            Ok(subdir) => v.push(subdir),
                            Err(e) => {
                                match e.kind() {
                                    MEK::NotAMaildirError   |
                                    MEK::CurDirDoesNotExist |
                                    MEK::NewDirDoesNotExist |
                                    MEK::TmpDirDoesNotExist => {
                                        /* ignored */
                                    },
                                    _ => return Err(e),
                                }
                            },
                        }

                    },

                    Err(e) => return Err(MEK::IOError.into_error_with_cause(Box::new(e))),
                }
            }
        }

        this.subdirs = Some(v);
        Ok(this)
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

