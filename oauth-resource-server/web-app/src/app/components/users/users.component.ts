import {Component, OnInit} from '@angular/core';
import {User} from '../../model/user/user';
import {DeleteUsersAction, FindUsersAction, UpdateUserAction} from '../../states/user.state';
import {Store} from '@ngxs/store';

@Component({
  selector: 'app-users',
  templateUrl: './users.component.html',
  styleUrls: ['./users.component.scss']
})
export class UsersComponent implements OnInit {

  users: User[];

  private currentUserId: string;

  constructor(private store: Store) {
    store.select(state => state.currentUser).subscribe(currentUser => this.currentUserId = currentUser.id);
  }

  ngOnInit(): void {
    this.store.dispatch(new FindUsersAction()).subscribe(_ => {
      this.store.select(state => state.users).subscribe(userList => {
        if (Array.isArray(userList.users)) {
          this.users = userList.users;
        }
      });
    });
  }

  setAdmin(admin: boolean, user: User): void {
    const newUser = new User();
    newUser.admin = admin;
    newUser.id = user.id;
    newUser.version = user.version;
    newUser.firstName = user.firstName;
    newUser.lastName = user.lastName;
    this.store.dispatch(new UpdateUserAction(newUser));
  }

  isActive(user: User): boolean {
    return this.currentUserId === user.id;
  }

  delete(user: User): void {
    this.store.dispatch(new DeleteUsersAction(user));
  }

}
