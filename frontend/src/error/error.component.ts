import { Component, computed, input } from '@angular/core';

@Component({
  selector: 'app-error',
  imports: [],
  templateUrl: './error.component.html',
  styleUrl: './error.component.scss'
})
export class ErrorComponent {
  protected readonly errorCode = input.required<string>();

  protected readonly errorMessage = computed(() => {
    const errorCode = this.errorCode();
    switch (errorCode) {
      case '401':
        return 'Unauthorized (Key wrong)';
      case '404':
        return 'Resource not found';
      case '412':
        return 'Payload too large (File too large)';
      case '429':
        return 'Too many requests (Reached daily limit)';
      case '431':
        return 'Request Header Fields Too Large (File name or MIME type too long)';
      case '500':
      case '502':
        return 'Internal server error';
      default:
        return `An error occurred (${errorCode})`
    }
  });
}
