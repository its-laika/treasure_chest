import { CommonModule } from '@angular/common';
import { Component, computed, input, signal } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { MatChipsModule } from '@angular/material/chips';
import { MatIconModule } from '@angular/material/icon';
import { UploadResponse } from '../../http/http.models';

@Component({
    selector: 'tc-upload-information',
    imports: [
        CommonModule,
        MatButtonModule,
        MatIconModule,
        MatCardModule,
        MatChipsModule,
    ],
    templateUrl: './upload-information.component.html',
    styleUrl: './upload-information.component.scss'
})
export class UploadInformationComponent {
    readonly file = input.required<UploadResponse>();
}
