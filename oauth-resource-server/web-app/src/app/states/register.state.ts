import {Action, State, StateContext} from '@ngxs/store';
import {Injectable} from '@angular/core';
import {BaseUser} from '../model/user/base-user';
import {User} from '../model/user/user';

export class RegisterInitAction {
  static readonly type = 'RegisterInit';

  constructor(public user: BaseUser) {
  }
}

export class RegisterStateModel {
  registerForm: RegisterFormModel;
}

export class RegisterFormModel {
  model: RegisterModel;
  dirty: boolean;
  status: string;
  errors: any;
}

export class RegisterModel {
  firstName: string;
  lastName: string;
  identifier: string;
  version: number;
  registered: boolean;
}

@State<RegisterStateModel>({
  name: 'register',
  defaults: {
    registerForm: {
      model: {
        firstName: '',
        lastName: '',
        identifier: undefined,
        version: undefined,
        registered: false,
      },
      dirty: false,
      status: '',
      errors: {}
    }
  }
})
@Injectable({
  providedIn: 'root'
})
export class RegisterState {

  @Action(RegisterInitAction)
  init(ctx: StateContext<RegisterStateModel>, action: RegisterInitAction): void {
    const newFormModel = new RegisterFormModel();
    newFormModel.model = new RegisterModel();

    const user = action.user as User;
    if (user.type === 'User') {
      newFormModel.model.firstName = user.firstName;
      newFormModel.model.lastName = user.lastName;
      newFormModel.model.identifier = user.id;
      newFormModel.model.version = user.version;
      newFormModel.model.registered = true;
    } else {
      newFormModel.model.firstName = '';
      newFormModel.model.lastName = '';
      newFormModel.model.identifier = undefined;
      newFormModel.model.version = undefined;
      newFormModel.model.registered = false;
    }
    ctx.setState({
      registerForm: newFormModel
    });
  }
}
