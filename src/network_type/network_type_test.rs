use super::*;

#[test]
fn test_network_type_parsing_success() -> Result<(), Error> {
    let ipv4: IpAddr = "192.168.0.1".parse().unwrap();
    let ipv6: IpAddr = "fe80::a3:6ff:fec4:5454".parse().unwrap();

    let tests = vec![
        ("lowercase UDP4", "udp", ipv4, NetworkType::Udp4),
        ("uppercase UDP4", "UDP", ipv4, NetworkType::Udp4),
        ("lowercase UDP6", "udp", ipv6, NetworkType::Udp6),
        ("uppercase UDP6", "UDP", ipv6, NetworkType::Udp6),
    ];

    for (name, in_network, in_ip, expected) in tests {
        let actual = determine_network_type(in_network, &in_ip)?;

        assert_eq!(
            actual, expected,
            "NetworkTypeParsing: '{}' -- input:{} expected:{} actual:{}",
            name, in_network, expected, actual
        );
    }

    Ok(())
}

#[test]
fn test_network_type_parsing_failure() -> Result<(), Error> {
    let ipv6: IpAddr = "fe80::a3:6ff:fec4:5454".parse().unwrap();

    let tests = vec![("invalid network", "junkNetwork", ipv6)];
    for (name, in_network, in_ip) in tests {
        let result = determine_network_type(in_network, &in_ip);
        assert!(
            result.is_err(),
            "NetworkTypeParsing should fail: '{}' -- input:{}",
            name,
            in_network,
        );
    }

    Ok(())
}

#[test]
fn test_network_type_is_udp() -> Result<(), Error> {
    assert!(NetworkType::Udp4.is_udp());
    assert!(NetworkType::Udp6.is_udp());
    assert!(!NetworkType::Udp4.is_tcp());
    assert!(!NetworkType::Udp6.is_tcp());

    Ok(())
}

#[test]
fn test_network_type_is_tcp() -> Result<(), Error> {
    assert!(NetworkType::Tcp4.is_tcp());
    assert!(NetworkType::Tcp6.is_tcp());
    assert!(!NetworkType::Tcp4.is_udp());
    assert!(!NetworkType::Tcp6.is_udp());

    Ok(())
}
