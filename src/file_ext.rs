use crate::utils::lowercase;

#[derive(Copy, Clone)]
pub enum FileExt {
    Jpg, Png, Gif, Svg, M4a,
    Mp4, Webm, Mp3, Ogg, Wav
}

impl FileExt {

    pub fn mime_type(&self) -> Vec<u32> {

        match self {
            FileExt::Mp4 | FileExt::M4a => vec![109, 112, 52],  // into_v32("mp4")
            FileExt::Mp3 => vec![109, 112, 101, 103],  // into_v32("mpeg")
            FileExt::Webm => vec![119, 101, 98, 109],  // into_v32("webm")
            FileExt::Wav => vec![119, 97, 118],  // into_v32("wav")
            FileExt::Ogg => vec![111, 103, 103],  // into_v32("ogg")
            _ => unreachable!()
        }

    }

}

// it reads `jpg`, `jpeg`, `png`, `gif`, `svg`, `mp4`, `webm`, `mp3`, `ogg`, and `wav` case-insensitively
// file name (before the extension) must be at least one character
pub fn read_file_extension(v: &[u32]) -> Option<FileExt> {

    if v.len() < 5 {
        return None;
    }

    let index = v.len() - 1;

    // __g
    if lowercase(&v[index]) == 'g' as u32 {

        if lowercase(&v[index - 1]) == 'n' as u32 && lowercase(&v[index - 2]) == 'p' as u32 && v[index - 3] == '.' as u32 {
            Some(FileExt::Png)
        }

        else if lowercase(&v[index - 1]) == 'p' as u32 && lowercase(&v[index - 2]) == 'j' as u32 && v[index - 3] == '.' as u32 {
            Some(FileExt::Jpg)
        }

        else if lowercase(&v[index - 1]) == 'v' as u32 && lowercase(&v[index - 2]) == 's' as u32 && v[index - 3] == '.' as u32 {
            Some(FileExt::Svg)
        }

        else if lowercase(&v[index - 1]) == 'g' as u32 && lowercase(&v[index - 2]) == 'o' as u32 && v[index - 3] == '.' as u32 {
            Some(FileExt::Ogg)
        }

        else if lowercase(&v[index - 1]) == 'e' as u32 && lowercase(&v[index - 2]) == 'p' as u32 && lowercase(&v[index - 3]) == 'j' as u32 && v[index - 4] == '.' as u32 {
            Some(FileExt::Jpg)
        }

        else {
            None
        }

    }

    // __f
    else if lowercase(&v[index]) == 'f' as u32 {

        if lowercase(&v[index - 1]) == 'i' as u32 && lowercase(&v[index - 2]) == 'g' as u32 && v[index - 3] == '.' as u32 {
            Some(FileExt::Gif)
        }

        else {
            None
        }

    }

    // __m
    else if lowercase(&v[index]) == 'm' as u32 {

        if lowercase(&v[index - 1]) == 'b' as u32 && lowercase(&v[index - 2]) == 'e' as u32 && lowercase(&v[index - 3]) == 'w' as u32 && v[index - 4] == '.' as u32 {
            Some(FileExt::Webm)
        }

        else {
            None
        }

    }

    // __v
    else if lowercase(&v[index]) == 'v' as u32 {

        if lowercase(&v[index - 1]) == 'a' as u32 && lowercase(&v[index - 2]) == 'w' as u32 && v[index - 3] == '.' as u32 {
            Some(FileExt::Wav)
        }

        else {
            None
        }

    }

    // __a
    else if lowercase(&v[index]) == 'a' as u32 {

        if v[index - 1] == '4' as u32 && lowercase(&v[index - 2]) == 'm' as u32 && v[index - 3] == '.' as u32 {
            Some(FileExt::M4a)
        }

        else {
            None
        }

    }

    // __4
    else if v[index] == '4' as u32 {

        if lowercase(&v[index - 1]) == 'p' as u32 && lowercase(&v[index - 2]) == 'm' as u32 && v[index - 3] == '.' as u32 {
            Some(FileExt::Mp4)
        }

        else {
            None
        }

    }

    // __3
    else if v[index] == '3' as u32 {

        if lowercase(&v[index - 1]) == 'p' as u32 && lowercase(&v[index - 2]) == 'm' as u32 && v[index - 3] == '.' as u32 {
            Some(FileExt::Mp3)
        }

        else {
            None
        }

    }

    else {
        None
    }

}