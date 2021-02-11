import {Component, OnInit} from '@angular/core';

@Component({
  selector: 'app-register',
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.scss']
})
export class RegisterComponent implements OnInit {

  title = 'Sign up';

  btnSaveTitle = 'Send';

  constructor() {
  }

  ngOnInit(): void {
  }

}
