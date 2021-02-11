import {Component, OnInit} from '@angular/core';
import {Store} from '@ngxs/store';
import {User} from '../../model/user/user';
import {AuthenticationModel} from '../../states/authentication.state';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.scss']
})
export class DashboardComponent implements OnInit {

  loggedIn: boolean;

  admin: boolean;

  username: string;

  constructor(private store: Store) {
  }

  ngOnInit(): void {
    this.store.select(state => state.currentUser).subscribe(
      currentUser => {
        const user = currentUser as User;
        if (user.firstName !== undefined) {
          this.username = user.firstName;
        }
        this.admin = user.admin !== undefined && user.admin === true;
      });
    this.store.select(state => state.authentication).subscribe(auth => this.loggedIn = (auth as AuthenticationModel).loggedIn);
  }

}
