use crate::RenderOption;
use crate::ast::{
    doc_data::DocData,
    node::Node,
};
use crate::utils::into_v32;

pub fn sidebar_to_html(nodes: &Vec<Node>, toc_rendered: &Vec<u32>, render_option: &RenderOption, doc_data: &mut DocData) -> Vec<u32> {
    let mut result = vec![
        into_v32("<div id=\"mdxt-sidebar-toggle\" onclick=\"mdxt_toggle_sidebar()\"><span id=\"mdxt-sidebar-button-content\">≫</span></div><div id=\"mdxt-sidebar\"><div id=\"mdxt-sidebar-close-button\" onclick=\"mdxt_close_sidebar()\">&times;</div><div id=\"mdxt-sidebar-content\">")
    ];

    for node in nodes.iter() {
        node.to_html(toc_rendered, render_option, doc_data, &mut result);
    }

    result.push(
        into_v32("</div></div>")
    );

    result.concat()
}

/// You can also write your own version.
///
/// ```javascript
/// var mdxt_sidebar_open = false;
/// function mdxt_toggle_sidebar() {
///
///   if (mdxt_sidebar_open) {
///     mdxt_close_sidebar();
///   }
///
///   else {
///     document.getElementById("mdxt-sidebar").style.width = "var(--sidebar-width)";
///     document.getElementById("mdxt-sidebar-toggle").style.left = "var(--sidebar-width)";
///     document.getElementById("mdxt-sidebar-button-content").innerHTML = "≪";
///     mdxt_sidebar_open = true;
///   }
///
/// }
///
/// function mdxt_close_sidebar() {
///   document.getElementById("mdxt-sidebar").style.width = "0";
///   document.getElementById("mdxt-sidebar-toggle").style.left = "0";
///   document.getElementById("mdxt-sidebar-button-content").innerHTML = "≫";
///   mdxt_sidebar_open = false;
/// }
/// ```
pub fn sidebar_javascript() -> String {
"var mdxt_sidebar_open = false;
function mdxt_toggle_sidebar() {

  if (mdxt_sidebar_open) {
    mdxt_close_sidebar();
  }

  else {
    document.getElementById(\"mdxt-sidebar\").style.width = \"var(--sidebar-width)\";
    document.getElementById(\"mdxt-sidebar-toggle\").style.left = \"var(--sidebar-width)\";
    document.getElementById(\"mdxt-sidebar-button-content\").innerHTML = \"≪\";
    mdxt_sidebar_open = true;
  }

}

function mdxt_close_sidebar() {
  document.getElementById(\"mdxt-sidebar\").style.width = \"0\";
  document.getElementById(\"mdxt-sidebar-toggle\").style.left = \"0\";
  document.getElementById(\"mdxt-sidebar-button-content\").innerHTML = \"≫\";
  mdxt_sidebar_open = false;
}".to_string()
}