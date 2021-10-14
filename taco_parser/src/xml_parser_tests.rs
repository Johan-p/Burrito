#[cfg(test)]
mod xml_parser_tests {
    use super::super::*;

    // #[test]
    // fn test_parse_xml() {
    //     let folder = "/home/jl/hdd_drive/GW2/taco_markers/POIs/";
    //     let xml_file = "General__Tequatl.xml";
    //     let contents = read_to_string(OsPath::new(folder).join(xml_file)).unwrap();
    //     let xml_parsed = parse_xml(&contents);
    //     println!("{:#?}", xml_parsed.marker_category)
    // }

    #[test]
    fn test_parse_testxml() {
        let folder = "data/";
        let xml_file = "test.xml";
        let contents = read_to_string(OsPath::new(folder).join(xml_file)).unwrap();
        let xml_parsed = parse_xml(&contents);
        println!("{:#?}", xml_parsed)
    }

    #[test]
    fn test_construct_lookup() {
        let leaf = MarkerCategory {
            name: "level1".to_string(),
            icon_file: Some("abcd".to_string()),
            children: vec![],
            height_offset: None,
        };
        let level2_1 = MarkerCategory {
            name: "level2-1".to_string(),
            icon_file: Some("bbbb".to_string()),
            children: vec![leaf],
            height_offset: None,
        };
        let level2_2 = MarkerCategory {
            name: "level2-2".to_string(),
            icon_file: Some("cccc".to_string()),
            children: vec![],
            height_offset: None,
        };
        let level3 = MarkerCategory {
            name: "level3".to_string(),
            icon_file: Some("dddd".to_string()),
            children: vec![level2_1, level2_2],
            height_offset: None,
        };

        let expected: HashMap<String, String> = [
            ("level3".to_string(), "dddd".to_string()),
            ("level3.level2-2".to_string(), "cccc".to_string()),
            ("level3.level2-1".to_string(), "bbbb".to_string()),
            ("level3.level2-1.level1".to_string(), "abcd".to_string()),
        ]
        .iter()
        .cloned()
        .collect();

        let mut result = HashMap::new();
        construct_lookup_map(&mut result, level3, "".to_string());

        let lookup1 = "level3.level2-2";
        assert_eq!(expected.get(lookup1), result.get(lookup1));
        let lookup2 = "level3.level2-1.level1";
        assert_eq!(expected.get(lookup2), result.get(lookup2));

        assert_eq!(expected, result);
    }

    #[test]
    fn test_esape_xml_str() {
        let input = "a&b<c".to_string();
        let escaped = escape_xml_str(input);
        println!("{}", escaped);
    }

    // Disabling this test as no valid trl file to include for unit test
    // #[test]
    // fn test_process_taco_data_tekkit() {
    //     let folder = "data/".to_string();
    //     let xml_file = "test.xml".to_string();
    //     let result = process_taco_data(folder, xml_file);
    //     println!("{:#?}", result);
    // }

    // #[test]
    // fn test_process_taco_data_reactif() {
    //     let folder = "/home/jl/Documents/reaktif_taco/POIs/".to_string();
    //     let xml_file = "General__Tequatl.xml".to_string();
    //     let result = process_taco_data(folder, xml_file);
    //     println!("{:#?}", result);
    // }

    #[test]
    fn test_pois_suffix() {
        let folder = "/abc/efg/POIs".to_string();
        let stripped = strip_poi_subfolder(&folder);
        let expected = OsPath::new("/abc/efg");
        assert_eq!(stripped, expected);
    }
}