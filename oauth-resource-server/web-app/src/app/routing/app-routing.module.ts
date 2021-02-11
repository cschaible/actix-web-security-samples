import {NgModule} from '@angular/core';
import {RouterModule, Routes} from '@angular/router';
import {UsersComponent} from '../components/users/users.component';
import {AboutComponent} from '../components/about/about.component';
import {AccountComponent} from '../components/account/account.component';
import {RedirectGuard} from './redirect-guard';
import {environment} from '../../environments/environment';
import {RegisterComponent} from '../components/register/register.component';
import {DashboardComponent} from '../components/dashboard/dashboard.component';

const routes: Routes = [
  {path: '', component: DashboardComponent, pathMatch: 'full'},
  {path: 'account', component: AccountComponent},
  {path: 'about', component: AboutComponent},
  {path: 'users', component: UsersComponent},
  {path: 'register', component: RegisterComponent},
  {
    path: 'sign-up',
    component: RedirectGuard,
    canActivate: [RedirectGuard],
    data: {
      externalUrl: environment.idp
        + '/auth/realms/app/protocol/openid-connect/registrations?client_id=account&response_type=code&scope=email'
    }
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes, {enableTracing: false})],
  exports: [RouterModule]
})
export class AppRoutingModule {
}
