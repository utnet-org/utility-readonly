use crate::service::calculate;
#[test]
fn test_gcd()
{
    assert_eq!(calculate::gcd(1920,1080),120);
    assert_eq!(calculate::add_num(1024,768),256);
}

#[test]
fn test_add_1()
{
    assert_eq!(calculate::add_num(2,2),4);
}

#[test]
#[ignore]
fn test_add_2()
{
    assert_eq!(calculate::add_num(2,2),5);
}