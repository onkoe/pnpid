#![no_std] // get rid of standard library + no alloc

#[cfg(test)]
mod tests {
    use pnpid::company_from_pnp_id;

    #[test]
    fn still_works_no_std() {
        let name = "American Megatrends Inc";
        let id = "AMI";

        let got = company_from_pnp_id(id).unwrap();
        assert_eq!(name, got);
    }
}
