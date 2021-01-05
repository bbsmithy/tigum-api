pub async fn send_evervault_verify_email() {

    // curl --request POST \
    //  --url https://cage.run/tigum-signup-cage \
    //  --header 'api-key: ' \
    //  --header 'content-type: application/json' \
    //  --data '{
	//    "name": "eyJpc3MiOiJldmVydmF1bHQiLCJ2ZXJzaW9uIjoxfQ.eyJjYWdlRGF0YSI6Ik9ZbDhqV244MEI2L3h4MVlBOXRnRlJrNEp6SUpCSzBjRWVQUVdNK241cU5uMU9hbDBrci9GQzNJbzVsblV6QUN1VmRJSlJtZXM4azJPVG1wWVlJT2g1alJ2bWQvaS9sSkd1SEtjM1VIaUdlNzREd1BoRExMVzlBNS9XalQ4Wnk4RnpBUzhPWVMzWThNODA1RVVTYVowS2RpWStuTWtuam1jaTVIOTB1L2NMMmFlOVNjZEtQc3NvanVyT3htOVAxb0RLVjF3allteEMrMzJBN0FEL1hMajZPUFY2dDBUdGRncFkzNkhHT1E0MFhKTlp0MHBuckJ4QXNpbk1MWTE3VUdvTnV6WnlVZG1RODdrZEZLNkxqb1RTUlZZbWVsS0k0YnJEUmtMZEtvZ0xCd3pLUUFpeHk5TEIzOW9xT290UjRJTmp5WVFOTjhWckFNSEZoS2ZlQ2I5UT09Iiwia2V5SXYiOiJPZ2NmeXN2aExDamNYU1haRkxaMzlBPT0iLCJzaGFyZWRFbmNyeXB0ZWREYXRhIjoiR2NhRVRkYitlRDZUOXQ1S2lOMTcwSHpibkJqT1pjVUZ0WGJNdFhnZHhzM0k5TCthd1lHZHluOG5zZWc9In0.add6c660-d044-4022-bc71-460de6d14b1e"
    //  }'
    
    let body: &str = "{'to': 'briansmith.work578@gmail.com','type': 'verify'}";

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_email_verify() {
        let send_email = send_evervault_verify_email();
    }
}

