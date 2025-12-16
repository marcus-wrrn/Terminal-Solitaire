use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct FIGfont {
    pub header_line: HeaderLine,
    pub fonts: HashMap<u32, FIGcharacter>,
}

impl FIGfont {
    fn read_header_line(header_line: &str) -> Result<HeaderLine, String> {
        HeaderLine::try_from(header_line)
    }

    // fn read_comments(lines: &[&str], comment_count: i32) -> Result<String, String> {
    //     let length = lines.len() as i32;
    //     if length < comment_count + 1 {
    //         Err("can't get comments from font".to_string())
    //     } else {
    //         let comment = lines[1..(1 + comment_count) as usize].join("\n");
    //         Ok(comment)
    //     }
    // }

    fn extract_one_line(
        lines: &[&str],
        index: usize,
        height: usize,
        hardblank: char,
        is_last_index: bool,
    ) -> Result<String, String> {
        let line = lines
            .get(index)
            .ok_or(format!("can't get line at specified index:{index}"))?;

        let mut width = line.len() - 1;
        if is_last_index && height != 1 {
            width -= 1;
        }

        Ok(line[..width].replace(hardblank, " "))
    }

    fn extract_one_font(
        lines: &[&str],
        _code: u32,
        start_index: usize,
        height: usize,
        hardblank: char,
    ) -> Result<FIGcharacter, String> {
        let mut characters = vec![];
        for i in 0..height {
            let index = start_index + i as usize;
            let is_last_index = i == height - 1;
            let one_line_character =
                FIGfont::extract_one_line(lines, index, height, hardblank, is_last_index)?;
            characters.push(one_line_character);
        }
        Ok(FIGcharacter {
            characters,
        })
    }

    fn read_required_font(
        lines: &[&str],
        headerline: &HeaderLine,
        map: &mut HashMap<u32, FIGcharacter>,
    ) -> Result<(), String> {
        let offset = (1 + headerline.comment_lines) as usize;
        let height = headerline.height as usize;
        let size = lines.len();

        for i in 0..=94 {
            let code = (i + 32) as u32;
            let start_index = offset + i * height;
            if start_index >= size {
                break;
            }

            let font =
                FIGfont::extract_one_font(lines, code, start_index, height, headerline.hardblank)?;
            map.insert(code, font);
        }

        let offset = offset + 95 * height;
        let required_deutsch_characters_codes: [u32; 7] = [196, 214, 220, 228, 246, 252, 223];
        for (i, code) in required_deutsch_characters_codes.iter().enumerate() {
            let start_index = offset + i * height;
            if start_index >= size {
                break;
            }

            let font =
                FIGfont::extract_one_font(lines, *code, start_index, height, headerline.hardblank)?;
            map.insert(*code, font);
        }

        Ok(())
    }

    fn extract_codetag_font_code(lines: &[&str], index: usize) -> Result<u32, String> {
        let line = lines
            .get(index)
            .ok_or_else(|| "get codetag line error".to_string())?;

        let infos: Vec<&str> = line.trim().split(' ').collect();
        if infos.is_empty() {
            return Err("extract code for codetag font error".to_string());
        }

        let code = infos[0].trim();

        let code = if let Some(s) = code.strip_prefix("0x") {
            u32::from_str_radix(s, 16)
        } else if let Some(s) = code.strip_prefix("0X") {
            u32::from_str_radix(s, 16)
        } else if let Some(s) = code.strip_prefix('0') {
            u32::from_str_radix(s, 8)
        } else {
            code.parse()
        };

        code.map_err(|e| format!("{e:?}"))
    }

    fn read_codetag_font(
        lines: &[&str],
        headerline: &HeaderLine,
        map: &mut HashMap<u32, FIGcharacter>,
    ) -> Result<(), String> {
        let offset = (1 + headerline.comment_lines + 102 * headerline.height) as usize;
        let codetag_height = (headerline.height + 1) as usize;
        let codetag_lines = lines.len() - offset;

        if codetag_lines % codetag_height != 0 {
            return Err("codetag font is illegal.".to_string());
        }

        let size = codetag_lines / codetag_height;

        for i in 0..size {
            let start_index = offset + i * codetag_height;
            if start_index >= lines.len() {
                break;
            }

            let code = FIGfont::extract_codetag_font_code(lines, start_index)?;
            let font = FIGfont::extract_one_font(
                lines,
                code,
                start_index + 1,
                headerline.height as usize,
                headerline.hardblank,
            )?;
            map.insert(code, font);
        }

        Ok(())
    }

    fn read_fonts(
        lines: &[&str],
        headerline: &HeaderLine,
    ) -> Result<HashMap<u32, FIGcharacter>, String> {
        let mut map = HashMap::new();
        FIGfont::read_required_font(lines, headerline, &mut map)?;
        FIGfont::read_codetag_font(lines, headerline, &mut map)?;
        Ok(map)
    }

    pub fn from_content(contents: &str) -> Result<FIGfont, String> {
        let lines: Vec<&str> = contents.lines().collect();

        if lines.is_empty() {
            return Err("can not generate FIGlet font from empty string".to_string());
        }

        let header_line = FIGfont::read_header_line(lines.first().unwrap())?;
        // let _comments = FIGfont::read_comments(&lines, header_line.comment_lines)?;
        let fonts = FIGfont::read_fonts(&lines, &header_line)?;

        Ok(FIGfont {
            header_line,
            fonts,
        })
    }

    pub fn convert(&self, message: &str) -> Option<FIGure<'_>> {
        if message.is_empty() {
            return None;
        }

        let mut characters: Vec<&FIGcharacter> = vec![];
        for ch in message.chars() {
            let code = ch as u32;
            if let Some(character) = self.fonts.get(&code) {
                characters.push(character);
            }
        }

        if characters.is_empty() {
            return None;
        }

        Some(FIGure {
            characters,
            height: self.header_line.height as u32,
        })
    }
}

#[derive(Debug)]
pub struct HeaderLine {
    pub hardblank: char,
    pub height: i32,
    // pub baseline: i32,
    // pub max_length: i32,
    // pub old_layout: i32,
    pub comment_lines: i32,
    // pub print_direction: Option<i32>,
    // pub full_layout: Option<i32>,
    // pub codetag_count: Option<i32>,
}

impl HeaderLine {
    fn extract_signature_with_hardblank(
        signature_with_hardblank: &str,
    ) -> Result<(String, char), String> {
        if signature_with_hardblank.len() < 6 {
            Err("can't get signature with hardblank from first line of font".to_string())
        } else {
            let hardblank_index = signature_with_hardblank.len() - 1;
            let hardblank = signature_with_hardblank[hardblank_index..]
                .chars()
                .next()
                .unwrap();

            Ok((String::from(&signature_with_hardblank[..hardblank_index]), hardblank))
        }
    }

    fn extract_required_info(infos: &[&str], index: usize, field: &str) -> Result<i32, String> {
        let val = match infos.get(index) {
            Some(val) => Ok(val),
            None => Err(format!(
                "can't get field:{field} index:{index} from {}",
                infos.join(",")
            )),
        }?;

        val.parse()
            .map_err(|_| format!("can't parse required field:{field} of {val} to i32"))
    }

    fn extract_optional_info(infos: &[&str], index: usize, _field: &str) -> Option<i32> {
        if let Some(val) = infos.get(index) {
            val.parse().ok()
        } else {
            None
        }
    }
}

impl TryFrom<&str> for HeaderLine {
    type Error = String;

    fn try_from(header_line: &str) -> Result<Self, Self::Error> {
        let infos: Vec<&str> = header_line.trim().split(' ').collect();

        if infos.len() < 6 {
            return Err("headerline is illegal".to_string());
        }

        let signature_with_hardblank =
            HeaderLine::extract_signature_with_hardblank(infos.first().unwrap())?;

        let height = HeaderLine::extract_required_info(&infos, 1, "height")?;
        let _baseline = HeaderLine::extract_required_info(&infos, 2, "baseline")?;
        let _max_length = HeaderLine::extract_required_info(&infos, 3, "max length")?;
        let _old_layout = HeaderLine::extract_required_info(&infos, 4, "old layout")?;
        let comment_lines = HeaderLine::extract_required_info(&infos, 5, "comment lines")?;

        let _print_direction = HeaderLine::extract_optional_info(&infos, 6, "print direction");
        let _full_layout = HeaderLine::extract_optional_info(&infos, 7, "full layout");
        let _codetag_count = HeaderLine::extract_optional_info(&infos, 8, "codetag count");

        Ok(HeaderLine {
            hardblank: signature_with_hardblank.1,
            height,
            comment_lines,
        })
    }
}

#[derive(Debug)]
pub struct FIGcharacter {
    // pub code: u32,
    pub characters: Vec<String>,
    // pub width: u32,
    // pub height: u32,
}

impl fmt::Display for FIGcharacter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.characters.join("\n"))
    }
}

#[derive(Debug)]
pub struct FIGure<'a> {
    pub characters: Vec<&'a FIGcharacter>,
    pub height: u32,
}

impl<'a> FIGure<'a> {
    fn is_not_empty(&self) -> bool {
        !self.characters.is_empty() && self.height > 0
    }
}

impl<'a> fmt::Display for FIGure<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_not_empty() {
            let mut rs: Vec<&'a str> = vec![];
            for i in 0..self.height {
                for character in &self.characters {
                    if let Some(line) = character.characters.get(i as usize) {
                        rs.push(line);
                    }
                }
                rs.push("\n");
            }

            write!(f, "{}", rs.join(""))
        } else {
            write!(f, "")
        }
    }
}
