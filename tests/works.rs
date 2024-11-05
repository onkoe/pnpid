#[cfg(test)]
mod tests {
    use pnpid::company_from_pnp_id;

    #[test]
    fn ami() {
        let name = "American Megatrends Inc";
        let id = "AMI";

        let got = company_from_pnp_id(id).unwrap();
        assert_eq!(name, got);
    }

    #[test]
    fn lowercase_should_fail() {
        let id = "ovr";
        let got = company_from_pnp_id(id);

        assert!(got.is_none());
    }

    #[test]
    fn long_names_are_cool() {
        let id = "IPI";
        let name_very_long =
            "Intelligent Platform Management Interface (IPMI) forum (Intel, HP, NEC, Dell)";

        let got = company_from_pnp_id(id).unwrap();
        assert_eq!(got, name_very_long);
    }

    #[test]
    fn whoever_owns_boe_forgot_their_own_name() {
        let name_and_id = "BOE";

        let got = company_from_pnp_id(name_and_id).unwrap();
        assert_eq!(name_and_id, got);
    }

    #[test]
    fn unisys_has_like_85_names() {
        let ids = ["UNB", "UNC", "UNM", "UNO", "UNS", "UNT"];
        let name = "Unisys Corporation";

        for id in ids {
            let got = company_from_pnp_id(id).unwrap();
            assert_eq!(name, got);
        }
    }

    #[test]
    fn this_guy_forgot_to_make_a_website() {
        let id = "OTB";
        let name = "outsidetheboxstuff.com";

        let got = company_from_pnp_id(id).unwrap();
        assert_eq!(name, got);
    }

    #[test]
    fn all_names_trimmed() {
        let id = "OTM";
        let name = "Optoma Corporation"; // without the `          ` originally included

        let got = company_from_pnp_id(id).unwrap();
        assert_eq!(name, got);
    }

    /// ensure that the non-breaking space isn't retained.
    #[test]
    fn armstel_inc_non_breaking_space() {
        let id = "AMS"; // original contains a non-breaking space! (`\u{a0}`)
        let name = "ARMSTEL, Inc.";

        let got = company_from_pnp_id(id).unwrap();
        assert_eq!(got, name);

        // make sure it absolutely doesn't work
        let bad_nonbreaking_space_id = "AMS\u{a0}";
        let didnt_get = company_from_pnp_id(bad_nonbreaking_space_id);
        assert!(didnt_get.is_none());
    }

    /// this name has a slash
    #[test]
    fn cni_has_slash() {
        let id = "CNI";
        let name = "Connect Int'l A/S";

        let got = company_from_pnp_id(id).unwrap();
        assert_eq!(got, name);
    }

    /// and this name has an ampersand (`&`)
    #[test]
    fn fel_has_ampersand() {
        let id = "FEL";
        let name = "Fellowes & Questec";

        let got = company_from_pnp_id(id).unwrap();
        assert_eq!(got, name);
    }

    /// another long name
    #[test]
    fn azh_long_name() {
        let id = "AZH";
        let name = "Shenzhen three Connaught Information Technology Co., Ltd. (3nod Group)";

        let got = company_from_pnp_id(id).unwrap();
        assert_eq!(name, got);
    }

    /// has an accent (the little tick above `é`)
    #[test]
    fn swc_has_accent() {
        let id = "SWC";
        let name = "Software Café";

        let got = company_from_pnp_id(id).unwrap();
        assert_eq!(name, got);
    }
}
