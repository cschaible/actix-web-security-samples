# Actix web security sample repository

This repo contains sample code demonstrating how to use the [actix-web-security](https://github.com/cschaible/actix-web-security) rust crate.

## Samples

* OAuth2 resource server + Web UI + Keycloak

## License

This sample code is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  [http://www.apache.org/licenses/LICENSE-2.0])
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [http://opensource.org/licenses/MIT])

at your option.  

Please read the license remarks regarging in the `oauth-resource-server/web-app` directory if you consider to re-use code from the web app.


<br/>

### Warning - Keycloak configuration is not production ready
The Keycloak configuration is only appropriate for testing on a local machine. The configuration is neither tought to be nor appropriate to run it in a public accessible or shared environment. Please read the Keycloak [Documentation](https://www.keycloak.org/documentation) how to configure Keycloak safely or use public available Identity Providers.

### Warning - Actix web security is neither audited nor penetration tested 
The actix-web-security crate is provided "as is" without warranties of any kind and is not verified to be secure. 
It has neither been audited to be safe in an audit nor been penetration tested. 
The crate was developed to the best of knowledge and belief.
It's in your own responsibility to check the code for potential security issues or bugs and your own decision 
whether you see the code as safe and trustworthy or whether you prefer to not use it.
The library is provided as open-source and the liability of any kind is excluded as described in the licenses
the software is provided under.