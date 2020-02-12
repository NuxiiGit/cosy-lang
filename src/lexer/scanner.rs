use crate::common::Context;
use crate::common::syntax::CharKind;

use std::rc::Rc;
use std::io::{ BufRead, BufReader, Lines };
use std::fs::File;
use std::collections::VecDeque;

/// A structure which reads characters of a file and returns individual `Context`s.
pub struct FileScanner {
    filepath : Rc<String>,
    lines : Option<Lines<BufReader<File>>>,
    line : usize,
    chars : VecDeque<char>,
    word : String
}
impl FileScanner {
    /// Creates a new scanner at this file path.
    pub fn open(filepath : &str) -> Option<Self> {
        if let Ok(file) = File::open(filepath) {
            Some(Self {
                filepath : Rc::new(filepath.to_string()),
                lines : Some(BufReader::new(file).lines()),
                line : 0,
                chars : VecDeque::new(),
                word : String::new()
            })
        } else {
            None
        }
    }

    /// Returns the kind of the next character.
    pub fn peek(&mut self) -> CharKind {
        if let Some(chr) = self.chr() {
            CharKind::identify(chr)
        } else {
            CharKind::EoF
        }
    }

    /// Returns the next character in the file, or `None` if you have reached the EOF.
    pub fn chr(&self) -> Option<char> {
        if self.lines.is_none() {
            None
        } else {
            if let Some(chr) = self.chars.front() {
                Some(*chr)
            } else {
                Some('\n')
            }
        }
    }

    /// Advances the scanner and adds the character to the word.
    pub fn next(&mut self) -> CharKind {
        self.advance(false)
    }

    /// Similar to `next`, except the character is ignored.
    pub fn skip(&mut self) -> CharKind {
        self.advance(true)
    }

    /// Advances the scanner.
    pub fn advance(&mut self, skip : bool) -> CharKind {
        if let Some(chr) = self.chr() {
            if let '\n' = chr {
                self.readln();
            } else {
                self.chars.pop_front();
            }
            if !skip {
                self.word.push(chr);
            }
            CharKind::identify(chr)
        } else {
            CharKind::EoF
        }
    }

    /// Returns the current substring.
    pub fn substr(&self) -> &str {
        &self.word
    }

    /// Clears the current substring.
    pub fn clear(&mut self) {
        self.word.clear();
    }

    /// Returns the current context for the current substring.
    pub fn context(&self) -> Context {
        Context {
            filepath : Rc::clone(&self.filepath),
            src : self.substr().to_string(),
            line : self.line
        }
    }

    /// Reads the next line of the file into the char queue.
    fn readln(&mut self) {
        if let Some(iter) = &mut self.lines {
            match iter.next() {
                Some(Ok(line)) => {
                    self.line += 1;
                    for x in line.chars() {
                        self.chars.push_back(x);
                    }
                },
                Some(_) => {},
                None => self.lines = None
            }
        }
    }
}