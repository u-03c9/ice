use super::*;

use crate::agent::agent_config::AgentConfig;
use crate::agent::Agent;
use std::time::UNIX_EPOCH;
use util::Error;

#[test]
fn test_candidate_priority() -> Result<(), Error> {
    let tests = vec![
        (
            CandidateBase {
                candidate_type: CandidateType::Host,
                component: AtomicU16::new(COMPONENT_RTP as u16),
                ..Default::default()
            },
            2130706431,
        ),
        (
            CandidateBase {
                candidate_type: CandidateType::Host,
                component: AtomicU16::new(COMPONENT_RTP as u16),
                network_type: AtomicU8::new(NetworkType::Tcp4 as u8),
                tcp_type: TcpType::Active,
                ..Default::default()
            },
            2128609279,
        ),
        (
            CandidateBase {
                candidate_type: CandidateType::Host,
                component: AtomicU16::new(COMPONENT_RTP as u16),
                network_type: AtomicU8::new(NetworkType::Tcp4 as u8),
                tcp_type: TcpType::Passive,
                ..Default::default()
            },
            2124414975,
        ),
        (
            CandidateBase {
                candidate_type: CandidateType::Host,
                component: AtomicU16::new(COMPONENT_RTP as u16),
                network_type: AtomicU8::new(NetworkType::Tcp4 as u8),
                tcp_type: TcpType::SimultaneousOpen,
                ..Default::default()
            },
            2120220671,
        ),
        (
            CandidateBase {
                candidate_type: CandidateType::PeerReflexive,
                component: AtomicU16::new(COMPONENT_RTP as u16),
                ..Default::default()
            },
            1862270975,
        ),
        (
            CandidateBase {
                candidate_type: CandidateType::PeerReflexive,
                component: AtomicU16::new(COMPONENT_RTP as u16),
                network_type: AtomicU8::new(NetworkType::Tcp6 as u8),
                tcp_type: TcpType::SimultaneousOpen,
                ..Default::default()
            },
            1860173823,
        ),
        (
            CandidateBase {
                candidate_type: CandidateType::PeerReflexive,
                component: AtomicU16::new(COMPONENT_RTP as u16),
                network_type: AtomicU8::new(NetworkType::Tcp6 as u8),
                tcp_type: TcpType::Active,
                ..Default::default()
            },
            1855979519,
        ),
        (
            CandidateBase {
                candidate_type: CandidateType::PeerReflexive,
                component: AtomicU16::new(COMPONENT_RTP as u16),
                network_type: AtomicU8::new(NetworkType::Tcp6 as u8),
                tcp_type: TcpType::Passive,
                ..Default::default()
            },
            1851785215,
        ),
        (
            CandidateBase {
                candidate_type: CandidateType::ServerReflexive,
                component: AtomicU16::new(COMPONENT_RTP as u16),
                ..Default::default()
            },
            1694498815,
        ),
        (
            CandidateBase {
                candidate_type: CandidateType::Relay,
                component: AtomicU16::new(COMPONENT_RTP as u16),
                ..Default::default()
            },
            16777215,
        ),
    ];

    for (candidate, want) in tests {
        let got = candidate.priority();
        assert_eq!(
            got, want,
            "Candidate({}).Priority() = {}, want {}",
            candidate, got, want
        );
    }

    Ok(())
}

#[test]
fn test_candidate_last_sent() -> Result<(), Error> {
    let candidate = CandidateBase::default();
    assert_eq!(candidate.last_sent(), UNIX_EPOCH);

    let now = SystemTime::now();
    let d = now.duration_since(UNIX_EPOCH)?;
    candidate.set_last_sent(d);
    assert_eq!(candidate.last_sent(), now);

    Ok(())
}

#[test]
fn test_candidate_last_received() -> Result<(), Error> {
    let candidate = CandidateBase::default();
    assert_eq!(candidate.last_received(), UNIX_EPOCH);

    let now = SystemTime::now();
    let d = now.duration_since(UNIX_EPOCH)?;
    candidate.set_last_received(d);
    assert_eq!(candidate.last_received(), now);

    Ok(())
}

#[test]
fn test_candidate_foundation() -> Result<(), Error> {
    // All fields are the same
    assert_eq!(
        (CandidateBase {
            candidate_type: CandidateType::Host,
            network_type: AtomicU8::new(NetworkType::Udp4 as u8),
            address: "A".to_owned(),
            ..Default::default()
        })
        .foundation(),
        (CandidateBase {
            candidate_type: CandidateType::Host,
            network_type: AtomicU8::new(NetworkType::Udp4 as u8),
            address: "A".to_owned(),
            ..Default::default()
        })
        .foundation()
    );

    // Different Address
    assert_ne!(
        (CandidateBase {
            candidate_type: CandidateType::Host,
            network_type: AtomicU8::new(NetworkType::Udp4 as u8),
            address: "A".to_owned(),
            ..Default::default()
        })
        .foundation(),
        (CandidateBase {
            candidate_type: CandidateType::Host,
            network_type: AtomicU8::new(NetworkType::Udp4 as u8),
            address: "B".to_owned(),
            ..Default::default()
        })
        .foundation(),
    );

    // Different networkType
    assert_ne!(
        (CandidateBase {
            candidate_type: CandidateType::Host,
            network_type: AtomicU8::new(NetworkType::Udp4 as u8),
            address: "A".to_owned(),
            ..Default::default()
        })
        .foundation(),
        (CandidateBase {
            candidate_type: CandidateType::Host,
            network_type: AtomicU8::new(NetworkType::Udp6 as u8),
            address: "A".to_owned(),
            ..Default::default()
        })
        .foundation(),
    );

    // Different candidateType
    assert_ne!(
        (CandidateBase {
            candidate_type: CandidateType::Host,
            network_type: AtomicU8::new(NetworkType::Udp4 as u8),
            address: "A".to_owned(),
            ..Default::default()
        })
        .foundation(),
        (CandidateBase {
            candidate_type: CandidateType::PeerReflexive,
            network_type: AtomicU8::new(NetworkType::Udp4 as u8),
            address: "A".to_owned(),
            ..Default::default()
        })
        .foundation(),
    );

    // Port has no effect
    assert_eq!(
        (CandidateBase {
            candidate_type: CandidateType::Host,
            network_type: AtomicU8::new(NetworkType::Udp4 as u8),
            address: "A".to_owned(),
            port: 8080,
            ..Default::default()
        })
        .foundation(),
        (CandidateBase {
            candidate_type: CandidateType::Host,
            network_type: AtomicU8::new(NetworkType::Udp4 as u8),
            address: "A".to_owned(),
            port: 80,
            ..Default::default()
        })
        .foundation()
    );

    Ok(())
}

#[tokio::test]
async fn test_candidate_marshal() -> Result<(), Error> {
    let tests = vec![
       (
            Some(CandidateBase{
                    network_type:       AtomicU8::new(NetworkType::Udp6 as u8),
                    candidate_type:      CandidateType::Host,
                    address:            "fcd9:e3b8:12ce:9fc5:74a5:c6bb:d8b:e08a".to_owned(),
                    port:               53987,
                    priority_override:   500,
                    foundation_override: "750".to_owned(),
                    ..Default::default()
            }),
            "750 1 udp 500 fcd9:e3b8:12ce:9fc5:74a5:c6bb:d8b:e08a 53987 typ host",
        ),
        (
            Some(CandidateBase{
                    network_type:   AtomicU8::new(NetworkType::Udp4 as u8),
                    candidate_type: CandidateType::Host,
                    address:       "10.0.75.1".to_owned(),
                    port:          53634,
                ..Default::default()
            }),
            "4273957277 1 udp 2130706431 10.0.75.1 53634 typ host",
        ),
        (
            Some(CandidateBase{
                    network_type:    AtomicU8::new(NetworkType::Udp4 as u8),
                    candidate_type:  CandidateType::ServerReflexive,
                    address:        "191.228.238.68".to_owned(),
                    port:           53991,
                    related_address: Some(CandidateRelatedAddress{
                        address: "192.168.0.274".to_owned(), 
                        port:53991
                    }),
                ..Default::default()
            }),
            "647372371 1 udp 1694498815 191.228.238.68 53991 typ srflx raddr 192.168.0.274 rport 53991",
        ),
        (
            Some(CandidateBase{
                    network_type:   AtomicU8::new(NetworkType::Udp4 as u8),
                    candidate_type:  CandidateType::Relay,
                    address:        "50.0.0.1".to_owned(),
                    port:           5000,
                    related_address: Some(
                        CandidateRelatedAddress{
                            address: "192.168.0.1".to_owned(), 
                            port:5001}
                    ),
                ..Default::default()
            }),
            "848194626 1 udp 16777215 50.0.0.1 5000 typ relay raddr 192.168.0.1 rport 5001",
        ),
        (
            Some(CandidateBase{
                    network_type:   AtomicU8::new(NetworkType::Tcp4 as u8),
                    candidate_type: CandidateType::Host,
                    address:       "192.168.0.196".to_owned(),
                    port:          0,
                    tcp_type:       TcpType::Active,
               ..Default::default()
            }),
            "1052353102 1 tcp 2128609279 192.168.0.196 0 typ host tcptype active",
        ),
        (
            Some(CandidateBase{
                    network_type:   AtomicU8::new(NetworkType::Udp4 as u8),
                    candidate_type: CandidateType::Host,
                    address:       "e2494022-4d9a-4c1e-a750-cc48d4f8d6ee.local".to_owned(),
                    port:          60542,
                ..Default::default()
            }),
            "1380287402 1 udp 2130706431 e2494022-4d9a-4c1e-a750-cc48d4f8d6ee.local 60542 typ host", 
        ),
        // Invalid candidates
        (None, ""),
        (None, "1938809241"),
        (None, "1986380506 99999999 udp 2122063615 10.0.75.1 53634 typ host generation 0 network-id 2"),
        (None, "1986380506 1 udp 99999999999 10.0.75.1 53634 typ host"),
        (None, "4207374051 1 udp 1685790463 191.228.238.68 99999999 typ srflx raddr 192.168.0.278 rport 53991 generation 0 network-id 3"),
        (None, "4207374051 1 udp 1685790463 191.228.238.68 53991 typ srflx raddr"),
        (None, "4207374051 1 udp 1685790463 191.228.238.68 53991 typ srflx raddr 192.168.0.278 rport 99999999 generation 0 network-id 3"),
        (None, "4207374051 INVALID udp 2130706431 10.0.75.1 53634 typ host"),
        (None, "4207374051 1 udp INVALID 10.0.75.1 53634 typ host"),
        (None, "4207374051 INVALID udp 2130706431 10.0.75.1 INVALID typ host"),
        (None, "4207374051 1 udp 2130706431 10.0.75.1 53634 typ INVALID"),
    ];

    let agent = Agent::new(AgentConfig::default()).await?;
    for (candidate, marshaled) in tests {
        let actual_candidate = agent.unmarshal_remote_candidate(marshaled.to_owned()).await;
        if let Some(candidate) = candidate {
            if let Ok(actual_candidate) = actual_candidate {
                assert!(
                    candidate.equal(&actual_candidate),
                    "{} vs {}",
                    candidate.marshal(),
                    marshaled
                );
                assert_eq!(marshaled, actual_candidate.marshal());
            } else {
                panic!("expected ok");
            }
        } else {
            assert!(actual_candidate.is_err(), "expected error");
        }
    }

    Ok(())
}
