// https://github.github.com/gfm/#links
// 밑에 함수들이 link_ref_defs까지 인수로 받아서, 걍 저 안에서 ref에 있나 검사해버리자
// 하는 김에 nested link가 있는지도 저 안에서 검사해버리자..!!

// [foo](address)
fn read_direct_link(content: &[u16], index: usize) -> Option<(Vec<u16>, Vec<u16>)> {  // Option<(link_text, link_destination)>
    todo!()
}

// [foo]
fn read_shortcut_reference_link(content: &[u16], index: usize) -> Option<Vec<u16>> {  // Option<link_text>
    todo!()
}

// [foo][bar]
// [foo][]
fn read_reference_link(content: &[u16], index: usize) -> Option<(Vec<u16>, Vec<u16>)> {  // Option<(link_text, link_label)>
    todo!()
}