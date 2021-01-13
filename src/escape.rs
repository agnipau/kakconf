/// Escape a string to be inserted raw-style like `: i${string}<esc>`.
pub fn escape_raw_insert(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();
    let mut offset = 0;
    for _ in 0..chars.len() {
        if chars[offset] == '<' {
            chars.insert(offset + 1, 'l');
            chars.insert(offset + 2, 't');
            chars.insert(offset + 3, '>');
            offset += 4;
            continue;
        }
        if chars[offset] == '>' {
            chars.insert(offset, '<');
            chars.insert(offset + 1, 'g');
            chars.insert(offset + 2, 't');
            offset += 4;
            continue;
        }
        offset += 1;
    }
    chars.iter().collect()
}

#[test]
fn test_escape_raw_insert() {
    let left = escape_raw_insert(
        r#"
# LSP integration.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾

eval %sh{kak-lsp --kakoune -s "${kak_session}"}

def lsp-restart %{ lsp-stop; lsp-start }

# set-option global lsp_completion<gt_trigger<gt "execute-keys ''h<lt>a-h><lt><gta-k>\S[^\s,=;*(){}\[\]]\z<lt>ret>''"
set global lsp_diagnostic_line_error_sign ''▓''
set global lsp_diagnostic_line_warning_sign ''▒''

# hook global WinSetOption filetype=(rust|python|dart|sh|typescript|javascript|html|css|json|go|c|cpp) %{
hook global WinSetOption filetype=(rust|typescript|javascript|python|sh|dart|json|css) %{
    map buffer us<gter ''l'' '': enter-user-mode lsp<lt>ret>'' -docstring ''LSP mode''
    map buffer<gt lsp ''R'' '': lsp-rename-prompt<lt>ret>'' -docstring ''rename symbol under cursor''
    lsp-enable-window
    hook -always global KakEnd .* lsp-exit
    # set window lsp_hover_anchor true
    face buffer DiagnosticError   "default+u"
    face buffer DiagnosticWarning "default+u"
    face buffer LineFlagErrors    "%opt{gruvbox_c9_red}"
}
"#.trim());
    let right = r#"
# LSP integration.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾

eval %sh{kak-lsp --kakoune -s "${kak_session}"}

def lsp-restart %{ lsp-stop; lsp-start }

# set-option global lsp_completion<lt>gt_trigger<lt>gt "execute-keys ''h<lt>lt<gt>a-h<gt><lt>lt<gt><lt>gta-k<gt>\S[^\s,=;*(){}\[\]]\z<lt>lt<gt>ret<gt>''"
set global lsp_diagnostic_line_error_sign ''▓''
set global lsp_diagnostic_line_warning_sign ''▒''

# hook global WinSetOption filetype=(rust|python|dart|sh|typescript|javascript|html|css|json|go|c|cpp) %{
hook global WinSetOption filetype=(rust|typescript|javascript|python|sh|dart|json|css) %{
    map buffer us<lt>gter ''l'' '': enter-user-mode lsp<lt>lt<gt>ret<gt>'' -docstring ''LSP mode''
    map buffer<lt>gt lsp ''R'' '': lsp-rename-prompt<lt>lt<gt>ret<gt>'' -docstring ''rename symbol under cursor''
    lsp-enable-window
    hook -always global KakEnd .* lsp-exit
    # set window lsp_hover_anchor true
    face buffer DiagnosticError   "default+u"
    face buffer DiagnosticWarning "default+u"
    face buffer LineFlagErrors    "%opt{gruvbox_c9_red}"
}
"#.trim();
    assert_eq!(left, right);
}

/// Doubles all occurences of `sub_string` in `master_string`.
pub fn double_string(master_string: &str, sub_string: &str) -> String {
    master_string.replace(sub_string, &format!("{0}{0}", sub_string))
}

