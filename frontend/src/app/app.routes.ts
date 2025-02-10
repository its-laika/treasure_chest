import { Routes } from '@angular/router';
import { UploadComponent } from '../upload/upload.component';
import { DownloadComponent } from '../download/download.component';
import { ErrorComponent } from '../error/error.component';

export const routes: Routes = [
    {
        'path': '',
        'component': UploadComponent
    },
    {
        'path': 'download',
        'component': DownloadComponent
    },
    {
        'path': 'error/:errorCode',
        'component': ErrorComponent
    },
    {
        'path': '**',
        'redirectTo': '/error/404'
    }
];
