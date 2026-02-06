use composite_firewall::{IpRule, Packet, PortRule, RuleGroup, SecurityRule};

#[test]
fn composite_firewall_tree_rejects_if_any_level_fails() {
    // Arbol:
    // root_group
    //  ├─ PortRule (bloquea 22)
    //  └─ ip_group
    //      ├─ IpRule (bloquea 10.0.0.8)
    //      └─ IpRule (bloquea 192.168.1.10)

    let mut ip_group = RuleGroup::new();
    ip_group.add_rule(IpRule::new("10.0.0.8"));
    ip_group.add_rule(IpRule::new("192.168.1.10"));

    let mut root_group = RuleGroup::new();
    root_group.add_rule(PortRule::new(22));
    root_group.add_rule(ip_group);

    // Caso 1: puerto bloqueado => rechazado aunque IP no esté bloqueada
    let p1 = Packet::new("8.8.8.8", 22);
    assert!(!root_group.check(&p1));

    // Caso 2: IP bloqueada => rechazado aunque puerto permitido
    let p2 = Packet::new("10.0.0.8", 80);
    assert!(!root_group.check(&p2));

    // Caso 3: todo permitido => aceptado
    let p3 = Packet::new("1.2.3.4", 80);
    assert!(root_group.check(&p3));
}
