#[test]
fn render_icon_test() {
    use std::fs::File;
    use std::io::{Read, Write};
    use crate::render_to_html_with_default_options;
    use crate::utils::{from_v32};
    use super::*;
    use super::render::*;

    let mut strings = vec![
        String::from("# Icons\n\n[[toc]]\n\n## Sizing\n\n"),
        String::from("`[[icon=github, size = 16]]`: [[icon=github, size = 16]]\n\n"),
        String::from("`[[icon=github, size = 32]]`: [[icon=github, size = 32]]\n\n"),
        String::from("`[[icon=github, size = 64]]`: [[icon=github, size = 64]]\n\n"),
        String::from("`[[icon=github, size = 128]]`: [[icon=github, size = 128]]\n\n"),
        String::from("## Coloring\n\n"),
        String::from("`[[red]]        [[icon=github]]        [[/red]]`: [[red]][[icon=github]][[/red]]\n\n"),
        String::from("`[[green]]      [[icon=github]]      [[/green]]`: [[green]][[icon=github]][[/green]]\n\n"),
        String::from("`[[blue]]       [[icon=github]]       [[/blue]]`: [[blue]][[icon=github]][[/blue]]\n\n"),
        String::from("`[[aqua]]       [[icon=github]]       [[/aqua]]`: [[aqua]][[icon=github]][[/aqua]]\n\n"),
        String::from("`[[emerald]]    [[icon=github]]    [[/emerald]]`: [[emerald]][[icon=github]][[/emerald]]\n\n"),
        String::from("`[[violet]]     [[icon=github]]     [[/violet]]`: [[violet]][[icon=github]][[/violet]]\n\n"),
        String::from("`[[pink]]       [[icon=github]]       [[/pink]]`: [[pink]][[icon=github]][[/pink]]\n\n"),
        String::from("`[[grassgreen]] [[icon=github]] [[/grassgreen]]`: [[grassgreen]][[icon=github]][[/grassgreen]]\n\n"),
        String::from("`[[gold]]       [[icon=github]]       [[/gold]]`: [[gold]][[icon=github]][[/gold]]\n\n"),
        String::from("## Alignments\n\n"),
        String::from("```\n"),
        String::from("[[center]]\n\n"),
        String::from("[[icon=github]]\n\n"),
        String::from("[[/center]]\n"),
        String::from("```\n\n"),
        String::from("[[center]]\n\n"),
        String::from("[[icon=github]]\n\n"),
        String::from("[[/center]]\n\n"),
    ];

    let mut icons = ICONS.iter().map(
        |(name, (_, src))|
        (name.clone(), src.clone())
    ).collect::<Vec<(Vec<u32>, usize)>>();  // (Name, Source)

    icons.sort_unstable_by_key(|(name, _)| name.clone());

    strings.push(String::from("## Icons Table\n\n"));
    strings.push(format!("Total {} icons.\n\n", icons.len()));

    for (name, _) in icons.iter() {
        strings.push(format!(" [{}](#icon{}) ", from_v32(name), from_v32(name)));
    }

    strings.push(String::from("\n\n"));

    for (name, src) in icons.iter() {

        let (source, license) = match src {
            src if *src == EVA_ICON => ("EVA", "MIT"),
            src if *src == MATERIAL_ICON => ("MATERIAL", "Apache 2.0"),
            src if *src == DEV_ICON => ("DEV", "MIT"),
            src if *src == ION_ICON => ("ION", "MIT"),
            src if *src == BOOTSTRAP_ICON => ("BOOTSTRAP", "MIT"),
            _ => unreachable!()
        };

        strings.push(format!(
            "\n\n[[box, inline, width = medium]]\n\n[[center]]\n\n[[icon = {}, size = 64]][[br]]{}[[br]][[anchor, id=icon{}]][[/anchor]][{}][[br]]{}\n\n[[/center]]\n\n[[/box]]",
            from_v32(name),
            from_v32(name),
            from_v32(name),
            source, license
        ));
    }

    strings.push(String::from("\n[EVA]: https://akveo.github.io/eva-icons"));
    strings.push(String::from("\n[ION]: https://ionic.io/ionicons"));
    strings.push(String::from("\n[DEV]: https://devicon.dev"));
    strings.push(String::from("\n[BOOTSTRAP]: https://icons.getbootstrap.com"));
    strings.push(String::from("\n[MATERIAL]: https://fonts.google.com/icons"));

    let raw_md = strings.concat();

    if crate::PRINT_TEST_PAGES { println!("\n\n{raw_md}\n\n"); }

    let raw_html = render_to_html_with_default_options(&raw_md);

    let mut f = File::open("./styles/markdown.css").unwrap();
    let mut css = String::new();
    f.read_to_string(&mut css).unwrap();

    let html = format!(
"
<!DOCTYPE html>
<html>
<head>
    <title>MDxt Icon Reference</title>
    <style>{}</style>
</head>
<body>
    <article class=\"markdown\">{}</article>
</body>
</html>
",
        css,
        raw_html
    );

    let mut f = File::create("icon_test.html").unwrap();
    f.write_all(html.as_bytes()).unwrap();
}