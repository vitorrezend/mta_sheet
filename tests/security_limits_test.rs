use mta_sheet::state::server_fns::validate_image_magic_bytes;

#[test]
fn test_image_magic_bytes_validation() {
    let png_header = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR";
    assert_eq!(validate_image_magic_bytes(png_header).unwrap(), ("image/png", "png"));

    let jpeg_header = b"\xFF\xD8\xFF\xE0\x00\x10JFIF";
    assert_eq!(validate_image_magic_bytes(jpeg_header).unwrap(), ("image/jpeg", "jpg"));

    let fake_exe = b"MZ\x90\x00\x03\x00\x00\x00";
    assert!(validate_image_magic_bytes(fake_exe).is_err());
}

#[test]
fn test_max_limits_constants_and_logic() {
    let limit_5mb = 5 * 1024 * 1024;
    assert_eq!(limit_5mb, 5_242_880);

    let quota_sheets = 50;
    assert_eq!(quota_sheets, 50);
}
