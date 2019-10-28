extern crate serde_json;
extern crate hcid;
extern crate hex;

static FIXTURES: &'static str = include_str!("../../test/fixtures.json");

fn test_correct(e: &hcid::HcidEncoding, id: &str, data: &[u8]) {
    assert!(!e.is_corrupt(id).unwrap());
    let r = e.decode(id)
        .unwrap_or_else(|err| panic!(format!(
            "correct test of {} failed: {:?}", id, err)));
    assert_eq!(data, r.as_slice());
    let r = e.encode(data).unwrap();
    assert_eq!(id, r);
}

fn test_correctable(e: &hcid::HcidEncoding, id: &str, data: &[u8], correct_id: &str) {
    assert!(e.is_corrupt(id).unwrap());
    let r = e.decode(id)
        .unwrap_or_else(|err| panic!(format!(
            "correctable test of {} failed: {:?}", id, err)));
    assert_eq!(data, r.as_slice());
    let r = e.encode(&r).unwrap();
    assert_eq!(correct_id, r);
}

fn test_errant_id(e: &hcid::HcidEncoding, id: &str, err: &str) {
    assert!(e.is_corrupt(id).unwrap());
    let r = e.decode(id).unwrap_err();
    assert_eq!(err, format!("{:?}", r));
}

fn test_errant_data(e: &hcid::HcidEncoding, data: &[u8], err: &str) {
    let r = e.encode(data).unwrap_err();
    assert_eq!(err, format!("{:?}", r));
}

fn test(e: &hcid::HcidEncoding, test: &serde_json::Value) {
    let test = test.as_object().unwrap();

    for t in test["correct"].as_array().unwrap().iter() {
        let id = String::from(t[0].as_str().unwrap());
        let data = hex::decode(&String::from(t[1].as_str().unwrap())).unwrap();
        test_correct(e, &id, &data);
    }

    for t in test["correctable"].as_array().unwrap().iter() {
        let id = String::from(t[0].as_str().unwrap());
        let data = hex::decode(&String::from(t[1].as_str().unwrap())).unwrap();
        let correct_id = String::from(t[2].as_str().unwrap());
        test_correctable(e, &id, &data, &correct_id);
    }

    for t in test["errantId"].as_array().unwrap().iter() {
        let id = String::from(t[0].as_str().unwrap());
        let err = String::from(t[1].as_str().unwrap());
        test_errant_id(e, &id, &err);
    }

    for t in test["errantData"].as_array().unwrap().iter() {
        let data = hex::decode(&String::from(t[0].as_str().unwrap())).unwrap();
        let err = String::from(t[1].as_str().unwrap());
        test_errant_data(e, &data, &err);
    }
}

#[test]
fn it_can_execute_fixtures() {
    let fixtures: serde_json::Value = serde_json::from_str(FIXTURES).unwrap();
    let fixtures = fixtures.as_object().unwrap();

    let e = hcid::HcidEncoding::with_kind("hck0").unwrap();
    test(&e, &fixtures["hck0"]);

    let e = hcid::HcidEncoding::with_kind("hca0").unwrap();
    test(&e, &fixtures["hca0"]);

    let e = hcid::HcidEncoding::with_kind("hcs0").unwrap();
    test(&e, &fixtures["hcs0"]);
}
