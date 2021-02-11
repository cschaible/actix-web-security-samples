import {AboutComponent} from './components/about/about.component';
import {AccountComponent} from './components/account/account.component';
import {AppRoutingModule} from './routing/app-routing.module';
import {AppComponent} from './components/app/app.component';
import {BrowserAnimationsModule} from '@angular/platform-browser/animations';
import {BrowserModule} from '@angular/platform-browser';
import {MdbModule} from 'mdb-angular-ui-kit';
import {NgModule} from '@angular/core';
import {OAuthModule} from 'angular-oauth2-oidc';
import {UsersComponent} from './components/users/users.component';
import {RegisterComponent} from './components/register/register.component';
import {environment} from '../environments/environment';
import {NgxsModule} from '@ngxs/store';
import {CurrentUserState} from './states/current-user.state';
import {AuthenticationState} from './states/authentication.state';
import {HTTP_INTERCEPTORS, HttpClientModule} from '@angular/common/http';
import {RegisterState} from './states/register.state';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {FormsModule, ReactiveFormsModule} from '@angular/forms';
import {NgxsFormPluginModule} from '@ngxs/form-plugin';
import {MatCardModule} from '@angular/material/card';
import {RedirectGuard} from './routing/redirect-guard';
import {NgxsStoragePluginModule, StorageOption} from '@ngxs/storage-plugin';
import {UserDetailsComponent} from './components/shared/user-details/user-details.component';
import {DashboardComponent} from './components/dashboard/dashboard.component';
import {MatSlideToggleModule} from '@angular/material/slide-toggle';
import {UserState} from './states/user.state';
import {MatButtonModule} from '@angular/material/button';
import {ErrorInterceptor} from './interceptor/error-interceptor';
import {MatSnackBarModule} from '@angular/material/snack-bar';

@NgModule({
  declarations: [
    AppComponent,
    AccountComponent,
    UsersComponent,
    AboutComponent,
    RegisterComponent,
    UserDetailsComponent,
    DashboardComponent
  ],
  imports: [
    AppRoutingModule,
    BrowserAnimationsModule,
    BrowserModule,
    FormsModule,
    HttpClientModule,
    MatButtonModule,
    MatCardModule,
    MatFormFieldModule,
    MatInputModule,
    MatSnackBarModule,
    MdbModule,
    NgxsFormPluginModule.forRoot(),
    NgxsModule.forRoot([
      CurrentUserState,
      AuthenticationState,
      RegisterState,
      UserState
    ], {
      developmentMode: !environment.production
    }),
    NgxsStoragePluginModule.forRoot({
      storage: StorageOption.SessionStorage
    }),
    OAuthModule.forRoot({
        resourceServer: {
          allowedUrls: [environment.api],
          sendAccessToken: true
        }
      }
    ),
    ReactiveFormsModule,
    MatSlideToggleModule
  ],
  providers: [RedirectGuard,
    {
      provide: HTTP_INTERCEPTORS,
      useClass: ErrorInterceptor,
      multi: true
    }],
  bootstrap: [AppComponent]
})
export class AppModule {
}
