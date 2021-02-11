import {Component, ViewChild} from '@angular/core';
import {AuthenticationService} from '../../services/authentication.service';
import {Store} from '@ngxs/store';
import {AuthenticationModel} from '../../states/authentication.state';
import {User} from '../../model/user/user';
import {MdbCollapseDirective} from 'mdb-angular-ui-kit';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'OAuth Sample Application';

  public loggedIn: boolean;

  public registered: boolean;

  public admin: boolean;

  @ViewChild('navbarToggler') toggler: MdbCollapseDirective;

  constructor(private authenticationService: AuthenticationService, private store: Store) {
    store.select(state => state.currentUser).subscribe(
      currentUser => {
        if (currentUser instanceof User) {
          const user = currentUser as User;
          this.admin = user.admin !== undefined && user.admin === true;
          this.registered = user.id !== undefined;
        } else {
          this.admin = false;
          this.registered = false;
        }
      });
    store.select(state => state.authentication).subscribe(auth => this.loggedIn = (auth as AuthenticationModel).loggedIn);
  }

  public login(): void {
    this.authenticationService.login();
  }

  public logout(): void {
    this.authenticationService.logout();
  }

  toogleMenu(): void {
    const collapsed = this.toggler.collapsed;
    if (!collapsed) {
      this.toggler.toggle();
    }
  }
}
