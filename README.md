<div align="center">
  <h1>Actix web security sample repository</h1>
</div>

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
The actix web security library is not verified to be secure. It has neither been audited to be safe in an audit nor been pentested.
The library was developed to the best of knowledge and belief.  
It's in your own responsibility to check the code for potential security issues or bugs and your own decision 
whether you see it as safe and trustworthy or whether you prefer to not use it.

In other words (excerpt of the MIT license):

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.