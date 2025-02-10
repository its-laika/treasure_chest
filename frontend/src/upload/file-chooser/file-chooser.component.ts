import { CommonModule } from '@angular/common';
import { Component, ElementRef, inject, output, signal, viewChild } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';

@Component({
    selector: 'tc-file-chooser',
    imports: [
        CommonModule,
        MatButtonModule,
        MatIconModule,
    ],
    templateUrl: './file-chooser.component.html',
    styleUrl: './file-chooser.component.scss'
})
export class FileChooserComponent {
    protected fileInput = viewChild<ElementRef>('fileInput')
    protected readonly file = output<File>();
    protected readonly hasFile = signal<boolean>(false);

    protected onClick() {
        this.fileInput()?.nativeElement.click();
    }

    protected onFileSelected(event: Event) {
        event.preventDefault();

        const files = (event.target as HTMLInputElement).files ?? [];
        const file = files.length > 0 ? files[0] : null;

        if (!file) {
            return;
        }

        this.file.emit(file);
        this.hasFile.set(true);
    }
}
