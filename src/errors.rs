use flexar;

flexar::compiler_error! {
    [[Define] SyntaxError]
    (SY001) "unexpected character": ((1) "character `", "` is unexpected");
    (SY002) "non-terminated string": "expected `\"` to terminate string";
    (SY003) "invalid escape character": ((1) "escape character `", "` is invalid");
    (SY004) "expected atom": ((1) "expected atom, found `", "`");
    (SY005) "expected path": ((1) "expected path, found `", "`");
    (SY006) "expected path-identifier after dot": ((1) "expected path-ident, found `", "`");
    (SY007) "expected stmt": ((1) "expected stmt, found `", "`");
    (SY008) "expected asignment operator": ((1) "expected `:` or `=`, found `", "`");
    (SY009) "all paths must begin with identifer": ((2) "try using `\"", "\"` instead of `", "`");
}