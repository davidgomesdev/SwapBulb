use std::env;
use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;
use std::time::Duration;

use serde_json::Value;
use tiny_http::{Header, Response, Server};

const TCP_PORT: u16 = 42424;
const ICON: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGQAAABkCAYAAABw4pVUAAAQAElEQVR4nOxdCXRV5bXeZ7jzzUBIIARCYgBBAQ0ag4AgClWxr8V5etLW2vccWqy12K5S7auzfU9rHetc5+qqPpwVKzKDxDAmQEIg0yVkHu48nKl7n3sTmZLc6VxObL61/rXPHc74/cP+9/AfHoahK/AwDF1hmBCdYZgQnYGFoQkzljT4DmIoEsIgKrE4cNsA3zEwMPRgQDJCtKEoSjoKN3yHMDyG6AzDhOgMQ3VQTzU4LIWQAgwTMjhG45i1GksNbp8JGiMVhHCQXCiHSQm0RQkS8Q3KuRBWHlygMZL9sI5GId7QPpZlp6JG9D4kBzIej8XjfYTb60A7zMFr/wplNpZdeL6FKPeDxtBa7T0Db2orygN4QxNh6GAsXncFyhFYPsFrvxqlF1IArbss6nepazkJSxYMEWAD/DOEyShDMq6AFJGhnhu0BfW7WyPnuRKGDmis6EAyfowyAClEKmbqt2PzfwzlFrzBs2EYAyIVhGRH7E5mJOQ0lBUwjH6htZZF8GGffCrK6Si7kJSvYBj9IiUTQ1mWPySJZJwFwxgQqbJl7Y7IcRA/qHsdA2GNrRBLHra4TAj7Ruz97CNgcWKF6EFJpQvLISx1WFpAh0il+f1yLFXwLTmDga5tOpYFOAadj3IelnRIHvxY6si3goRtwu0vsOyFEww9+kPIE3gTPqj/Qnny4T+wDLSk25mWTDvjzbSBmJXOSlk2UGxmtq+lWyyywBsZqcetmNxekFweYNw+mXN6wdDhVGzugDJCFCEX/2o8zrnLsVt9FuWrWEQ4AdATIfRQlyIRyyFsrkDXIHScOZmrvOFCI3/5PH5Szgh2NCQBigJiQ4vcvGaHePDDTYJ3Vbk0xhNUToFvx9QqJOYGlF9DiqEXQiYjEW+hPIM+mI1Qc/cS46E7rracbeTBBCnAXodU++snA/Ury8WZ+NGGRURS7kD5JKQQeiDkIiTjHQiPD94fXchvfn6Zba6BSw0RR6OhVWk6/3aPo75VViexSMofUNwHKcKJJuQCJOMDlOYMG7Oj/DlbZlEeVwgnGLIMyvX3e9e+s0Y8Fz8ySMpPUf4NUoATSch8JOMTlNb8HHZ91Wv2YrOJ0VVozxX/41+7Yn2ISPEjKSUo94DGOFGETEAytqFML5nMfbXpafs8jtWff59ayqTr3WX1LTKNK+uQlPnwrYNME5wIQoxIxkaUJaMz2LLGd9Nm8Jx+46scbXLbSde60Q4H6UjIxfjVZ6AhUu5Tx9n1wyhKeBbqK1+1n6RnMgj5o9hRl59j3E7bWJFuB42R6hYyE2+KZsXy+ifte2dP5abDEECHU3aOvtRNJhojthK65mitDTEjlS2ERTKeIDmvmF83VMggZGewGQW57E7cZLCFXwcaIpWE3ISllMwfK+61FsMQw62XGmSS2EIuAg2RKkIs2DpoggUP/NS8D21RQ8a/3otb/sNIzjVy587AkgsaIVWE3Igl18TD/mXXmebAEITNwlpH2JkDEB53LwCNkBJCsHUQIXDHNSYHdlmp8FJqgqlFXDdJHEc0i2BMBSFTsNCYEVx2tWnIjR2HY8HpjPq8IpqWJhiIkNlYJkHiUJt37kh2R6aNGQFDGOcWG0ZGNqeBRujPXHEKdjMbUPZgbSBPXSXECTwOxcXCghmsB4Y4JubzOZHNnEhpP+ovpKxQhObIw4o58psTws+xDEuov3P0RwhFHK7Gcj4+0M+QlPMj38UDtXkvnmPQ1HAoSiCXV4sHcIyCMybzRWgJSPpYlZultnDy05N1gZxlhxNCmV006GcOcpj3ItGQx0V/hJBz5jI8wZcQjgBfi58XQOw+Z3ooFJQAJVP4pHj7joe3vxJ2/uhBX54kh7tYJKXzxTsttT++yJjUKBc0gDIcB22SBGPh2NBYImo9FlKPW7F04HPrgEjkIz4/CtCYSjFquN3vOQYznWTgAT6F8HjSHOm+YokAH4X708UJoVUZCscc14+dEOqbpfYJ/+kh5xY5tLzo9g3i/WbhjXlb308TR6azGZBE2Bc5a/1BKKIKix9XQJIxmJblxBMvQrkFyxh8uHQBNoge6iCOD8mpBRmEB94MVaMwjbQzu9yfp7PBf2ZkYi1uwjpoq6yTD/7yCf/GnEtclROXuLds3ScmnE6Az6A3+CGpRPciGrXXhaQshnA8E2kXV0H0UN2w2IVoFrCMvgq1/S8s4bqsRsZSc1Buwi6FukfhmRXBzqfeD83pcinT6prkmfe8Emyk/zZ3Kt3LX/Rt2rRb2AcxAitWb5KQJlEp0c5DWokUrB1PQWzNtPfiE7Yq51/u2mG+wOlo7lKO0GzG5YRvYU+jbPAGZd/sn3socp2fXsRu+mCjeEQYUckUjnnzS2FH/pUu05/eEmbPXeorQEUgJlJYru+egqABYpkYlsuyvBTCEYDRQs2rwDpsgQSwZa9Qd6hbKRZEyC++0X0QZV/tvGERP4pkZa1cPOlaT5XTq0zFFtn++nJrgSAdYXMKnFrApePgPxmblDXynWnnAbkTYoAk9ilCmhCitdu0GVQFA9Lx4YQMXHzjCMsyfWpJh1OZcc5S98bbrzRb8Xhst0uWWBY60d06srVHoTAi78pHbIfGZjNHTGrnTOU3L3nQV4SbfZUDtabm6xYaTocY4A8pvWOHJgsWaO6gwm6uDUVO1av2hkn5XAHEiRk/c2/YVSufE81/8UG3oQpM8x5L+BrAOe80vmLtTvHw/f2fPmyturDUMAOiREgA0XKhkyqHAWtZPsqDkGSkwpZFygDsqZPbIQFsfT5tzhu/s3yDFtedvd9hCzloNTP7Tsply266xLhuYp6azygjGdSNERnU3+OcCjIOJ4MiIt/9o6UiFjII1Q6JCKBJIXXFTaABUhHpUY/l9O0HRO/iefG7z7FbYq79nvGs0lO5Qycv8ZDpwfjackvXVeepfooILKh1Sc2fl0mN9P/vl/KFVQ6p7qLf+k6SwyT5Jo1lt376v9bxRWO4UogRq7cLFDFfiIUUAU2iTzQnBJv2Vuy2Fm/bJ8uQBEwYy+XNmsqv27xbnPerpwMSEnLE74W53Jibf8iN6f1sMjMcnlm1J618xFq98AzDXIgTKzZIqg0K72fXQLPtRJCKLosS72FLlZS0idQTt5nIdAEtXUpxZZ3YONB/f/tX3y4UtrHZ7NdIRkxd1NEo2yOq+S2oba4GjZAqQhTUjk4OhJSkTBDPmMRPyLAzZDll7ns1WN/f/9DgKL7+pahqW0/cZk7I2Lhtn1AXEIC0NGrpX4BGSAUhpOdToqf9nTXCTkgSLpltoGwo+Hiz2K/m9urK0Dc4ax9nMkINWptLIAHc95roiGzS6hHNoBFS5cJVcwwf/0cwaZOpmxcb8khirS3Y55COm5729IogWWBh0Uy+ETWruFV8nHsEP94cmkrbOHa8AhoiVUmfb6BQcFZ8Vmu30gFJwFlT+Ik431CJeGNVsO7o3yUFpIpamZJw4FeXmUZBArj75UA5WhvI2UTnexs0RKqiTsgiSwOh5eY/+5KSp441HmZMYskhBB9vOnZRoFVbhQp8iDlIWtPs6fxUiBMun+J//N2QahPD1kFhsJqYTHqRskA5vJl7SX64UZy1u16qgyRgUSmndkP7HMoxLeDFj0LqGFM8iavBOUnc97l4uW87qs3kriXy/woaI5WRi2ux0BJN5rlLvV7sl/2QIL5XYlBzEf1BpcAXVI7wU3+9V1YNiIvOin+q9cwHga3rdonknJOxQlEoUwg0Rkqj3/GmbgGyD3qVacU3enagwTGhBcjOnGwg9zAN3Pz+JqX18N+au+RCkj88xxCX6/ijzaE9Sx8P0goUpJT8CcIVKhpcwrLsmxBnCneq0xFakJRLUPr3N8mzCq507WrvUbogTpiNYDDwYRW0xiH2uQUc7dJB7GbI9O4qnsjFvE4XalS7L73LPx7C9rAPUCm5K9p9kbxb8R6v603VgxiRbELMMPhFbIyQ4kJz+YyxV7iCz34Y2gJxAo2NajThngaxb02rL9GWRTIzjanFQT0mA9oLnwS3L/69fwJaRmh1CIq4uQbCk8GoEMncpUo2H1tKzBm8ySKEBtffYa3oxEJW0MEixL/AC6e+eRdaZsf8/C/+mYVXu8sOHJJiNmfn5TAqETUORuj97rMyQR2fpoxlY3GmwUNv+rfd/GiAjJVUqT7Ca7wUYl8vqzLi8hZQ/gzlZbHsnBRCsCbci0Q8CGFPHIXHRGMz2k2JlFhooQC/o10unXy9J/OhN/wbIAaMHcmr41Bdi9x3L+U1suoHKZ3GRG0B/OWTvs13vRSi6yYTy/9HYqfiVXE34P5/pA18LvdDDM85GYTchienPhaFcjMWuqmHo9yXavVDkXW01uHTs9/1cuici3/jXRPl/jAyYrLscsl9XdOhNll13S443RjVwHrrY75NT60QZkF4Xfm/R9ZYTFSj+guE47Nocrow2p0SJeS/8QboxMTGr1E8h2UHxO4r2I/7n9dL7Mpycf5V90RHyujM8D34BUU1HqIK7BRkUO1bs0/jxw+2/2PvBr557iOVDGLjFRzAl0ByIkp8tN5vZHtWtDslRAie8JGwYB5F+RgkBho4H0BSfoJSeW+tOBe9fIPm8o1ICxMRCoZ9O+U16qSTI395VhqTM9C+FfWyY9kzQQp1pTHwMySD+vykrQWMx+tdrC1qb2lChODDIxXvFjzxMkgeXkOCX0DJXX2Pb9AB1WRk1XvAOY1KzOqtohpFkpPJOgbaj/xLC+/w0KBPA3hlRJtK9sLML+BxaUx9JtodEu2yyGj4LCQZSDClvwVwjnJmWZVYPdB/zaawFTckhtXblg5J7S5LT2EHXNr1i3KhqqNHzfOQaN4A2q1a3R3Ln3X7/hDU3F6hZVrnTOPXrHvCNr/3e5dX8dcektv2NsrdGypE76ebBXNju0IZTX6eVwOhDVj783DC6DDzjNdshqCBA8VqZvxpVlAKc3nh7GmM6ZVPBAaPQWnaf8cKoGlmbSzQ8wtdKBViFVp1e8xGpiMkKGk4ZyGdKubZ70BA0n+B4mnQCfRKyE8iYav9BXb7Od7YkDZyREdO4UR/7viJxszcXJPJlm7hGMaIfZYiS7In4HFzohjy+n1eJujuFrxOl+xxdsutB6ozfS4nxSnTvIkmcBSvnKy16ROCHgmhKPt6lEajyVIxoWRm67gp02wjc8fZrGmZNnN6ut1stdFAmdCSHO7OjsZ//N8fantamubjx3YkhVTklK5ifTzo8Q07FJRgRGXas+T+x/PSs0dNZ5jk1hsK4fF7XMyo8YUcEkLqdm+KmgNOMHS5CCYSQHMJNcGS5w0OzmgIKDItlsSoRj5ZFu2K/K2GyPG8CxiFpXUvgOVCRqM5aLTbBLM1XTIZjKqNS8Gd3d2dRq+rxxZwOwsiLxRTged7IzIhPOHQ6xgyGx8S+RQKQTvQnONLJIasC0nPhIoXutWyIlnAc9KzR39d8v3LQ1a7/biLJXMGk8xxuIYloAAAA+hJREFUjBLy+1i/1xcQgwEZuyMlJATFoMutBDwuNuDxcrbMTCEtK5sN+v3Knk1fURKrGHmnSQPoCHp9Sxt1R2rC5jW/f3iifURWNiQRB6srylyd7aXYw92EXdVy0BH0/FIwdca9Y/XnNZIoCpBEnH7eRarxEFuI5i/5ihV6XXdEwS6LluE4tal6d/43n7zX4uzu2JU1Ks9osafZIU6tq/lA9YGda1buNZltXH3lNorTpWT+50FH0PNM/W4k5d5jvmUYt8mW1pCemeHjTVbUpOwSA2FHFGviTHJQOsKpFBD8XKDHbfB0t2WFgoGJELZO09tzsiPm9htAR9ArIfRSLsrBsM5YcPFqBo1Ue9avygr4vDS7TqRVk2ZF3R+ZX3xICuWIaLZcXzzQKyG3IiFP40y98hfPvd230EswEHB1NtY5Dh2o6upsdohCMCiLvgAvyEKQIqdCQb8Vp/cB9IYoBtagpmQzBk62j8hmRo0rsBQVl0757LlHdzRWVdBavO9HfOa6gi61LNR+yNcORaeVHJEhazKb0/NOPmUqFogXxRf8IB0Joc1TQYfQpZaF/Xo9yeqtG0/559+e2ixJUlKyrwjBgK/XCeUEHUKvWtZO7LIuRr20qK2hNr+qbH25gTe080YTg2YRC8vz/VYkCTXkkN8PQsDv9PR0B/1uV1d32yGXq72tnTcZDeveeqnJ0905Ho//NrZCzRJv4oWetawFkdWIjgXD+FCzEvB3P9qozEgcLZhPoT8D5sHj/7si/7OgpPz0XaAz6PZ96jiOnEvjCJrad2aOznW2ORoKZFGk3EIeCbAqZC5UBl0AxoskkFYVYDguhPvTjJ8IoUA6GoiGCYkBaoZU/pRpXT+4bfl5tC0KYkgM+LsCfo8HZ+8BfNgszhFNCsMpZptdYSIr9BitVoZjOdo+wsGF/o91jt07aYkpSyT+inLo14OOoGdC1CUBPd1dfeMcb+CNvCEty5yWFte6v1feee+8ntbmhrfuW+YMeDwUnEfhrLoiRLe2LNS0NpNsbawtksRQ0rKWeLM5I+j19r6+L+blmbSGno2LFM7fIEvSuJfuvKUCrbNtkCAOVu+uefk3N7VHYqUovOhj0Bn0rGURZmFf/zmEk18k1mBosKeP6OINRllWJBlrujUY8NsVScrEIZ66Ng61ryAOID6W4wIcw5FfkVEkRRKEYA56HXvN+E2RNdzjXm1VK+idEMLkSMgqLTWY6LzJicd6HbtDeslXwi1OCwwFQnpBqWkUtEwJnqSMUAhPx2GF1FuKPqRolF4Nyx75TGMQeQapRSQ7XDSpGEqE/FtAz2rvvyWGCdEZhgnRGf4FAAD//49f2OoAAAAGSURBVAMAdjm24i8XwFsAAAAASUVORK5CYII=";

fn handle_client(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Bind UDP socket to any available local port
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
    socket.set_read_timeout(Some(Duration::from_secs(3)))?;

    // Step 1: get current state
    let get_pilot = r#"{"method":"getPilot"}"#;
    socket.send_to(get_pilot.as_bytes(), target)?;

    let mut buf = [0u8; 2048];
    let (len, _) = socket.recv_from(&mut buf)?;
    let response = std::str::from_utf8(&buf[..len])?;

    let json: Value = serde_json::from_str(response)?;
    let current_state = json["result"]["state"]
        .as_bool()
        .ok_or("Missing result.state")?;

    let new_state = !current_state;

    println!("Current state: {}", current_state);
    println!("Setting to: {}", new_state);

    // Step 2: set new state
    let set_state = format!(
        r#"{{"method":"setState","params":{{"state":{}}}}}"#,
        new_state
    );

    socket.send_to(set_state.as_bytes(), target)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html_response = format!(
        r#"<html>
    <head>
    <link rel="icon" type="image/x-icon" href="{}">
    </head>
    <body>
    <script>window.close();</script>
    </body>
    </html>"#,
        ICON
    );
    let target = format!("{}:38899", env::var("WIZ_BULB_IP").expect(""));
    println!("Listening on TCP port {}", TCP_PORT);

    let server = Server::http(format!("0.0.0.0:{}", TCP_PORT)).unwrap();

    for request in server.incoming_requests() {
        if !request.url().ends_with("/bulb") {
            if let Err(e) = request.respond(Response::empty(404)) {
                eprintln!("There was an error dummy responding: {:?}", e);
            }
            continue;
        }

        println!(">> Client connected");

        if let Err(e) = handle_client(&target) {
            eprintln!("There was an error setting bulb: {:?}", e);
            continue;
        }

        let response = Response::from_string(&html_response)
            .with_header(Header::from_str("Content-Type: text/html").unwrap());

        if let Err(e) = request.respond(response) {
            eprintln!("There was an error responding: {:?}", e);
            continue;
        }

        println!("responded")
    }

    Ok(())
}
