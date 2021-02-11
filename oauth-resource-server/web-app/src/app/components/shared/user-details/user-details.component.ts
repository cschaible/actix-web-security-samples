import {Component, Input, OnInit} from '@angular/core';
import {FormControl, FormGroup, Validators} from '@angular/forms';
import {Router} from '@angular/router';
import {Store} from '@ngxs/store';
import {RegisterInitAction, RegisterStateModel} from '../../../states/register.state';
import {BaseUser} from '../../../model/user/base-user';
import {User} from '../../../model/user/user';
import {DeleteCurrentUserAction, LoadUserAction, RegisterUserAction, UpdateCurrentUserAction} from '../../../states/current-user.state';

@Component({
  selector: 'app-user-details',
  templateUrl: './user-details.component.html',
  styleUrls: ['./user-details.component.scss']
})
export class UserDetailsComponent implements OnInit {

  @Input() title: string;

  @Input() btnSaveTitle: string;

  @Input() btnDeleteTitle: string = undefined;

  @Input() btnDeleteEnabled = false;

  protected firstNameFormControl = new FormControl('', [Validators.required, Validators.maxLength(256)]);

  protected lastNameFormControl = new FormControl('', [Validators.required, Validators.maxLength(256)]);

  private idFormControl = new FormControl('', null);

  private versionFormControl = new FormControl('', null);

  private registeredFormControl = new FormControl('', null);

  public registerFormGroup = new FormGroup({
    firstName: this.firstNameFormControl,
    lastName: this.lastNameFormControl,
    id: this.idFormControl,
    version: this.versionFormControl,
    registered: this.registeredFormControl
  });

  constructor(private router: Router, private store: Store) {
  }

  ngOnInit(): void {
    this.store.dispatch(new LoadUserAction()).subscribe(_ => {
      this.store.select(state => state.currentUser).subscribe(currentUser => {
        this.store.dispatch(new RegisterInitAction(currentUser as BaseUser)).subscribe(_ => {
          const user = currentUser as User;
          if (user.type === 'User') {
            // Initialize form controls with existing values.
            this.firstNameFormControl.setValue(user.firstName);
            this.lastNameFormControl.setValue(user.lastName);
            this.idFormControl.setValue(user.id);
            this.versionFormControl.setValue(user.version);
            this.registeredFormControl.setValue(true);
          } else {
            this.firstNameFormControl.setValue('');
            this.lastNameFormControl.setValue('');
            this.idFormControl.setValue(undefined);
            this.versionFormControl.setValue(undefined);
            this.registeredFormControl.setValue(false);
          }

          // Validate values
          Object.keys(this.registerFormGroup.controls).forEach(field => {
            const control = this.registerFormGroup.get(field);
            if (control instanceof FormControl) {
              control.markAsTouched({onlySelf: true});
            }
          });
        });
      });
    });
  }

  sendRegistration(): void {
    if (this.registerFormGroup.valid) {
      // Take current snapshot when the button is clicked and update the current user
      const currentUser = (this.store.snapshot().register as RegisterStateModel).registerForm.model;
      if (currentUser.registered === undefined || currentUser.registered === false) {
        this.store.dispatch(new RegisterUserAction(currentUser.firstName, currentUser.lastName)).subscribe(u => {
          this.store.dispatch(new RegisterInitAction(u as BaseUser)).subscribe(ignored => {
            this.router.navigateByUrl('/');
          });
        });
      } else {
        this.store.dispatch(new UpdateCurrentUserAction(currentUser.firstName, currentUser.lastName)).subscribe(u => {
          this.store.dispatch(new RegisterInitAction(u as BaseUser)).subscribe(ignored => {
            this.router.navigateByUrl('/');
          });
        });
      }
    }
  }

  deleteAccount(): void {
    const currentUser = (this.store.snapshot().register as RegisterStateModel).registerForm.model;

    if (currentUser.registered === undefined || currentUser.registered === false) {
      this.router.navigateByUrl('/');
    } else {
      this.store.dispatch(new DeleteCurrentUserAction()).subscribe(_ => {
        this.router.navigateByUrl('/');
      });
    }
  }

}
