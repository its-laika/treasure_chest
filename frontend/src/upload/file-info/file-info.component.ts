import { CommonModule } from '@angular/common';
import { Component, input, } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatCardModule } from '@angular/material/card';
import { MatChipsModule } from '@angular/material/chips';
import { ToReadableBytesPipe } from '../../conversion/to-readable-bytes.pipe';

@Component({
    selector: 'tc-file-info',
    imports: [
        CommonModule,
        MatButtonModule,
        MatIconModule,
        MatCardModule,
        MatChipsModule,
        ToReadableBytesPipe],
    templateUrl: './file-info.component.html',
    styleUrl: './file-info.component.scss'
})
export class FileInfoComponent {
    public readonly file = input.required<File>();
}
