import { HttpClient, HttpHeaderResponse, HttpHeaders } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { Configuration, UploadResponse } from './http.models';

@Injectable({
  providedIn: 'root',
})
export class HttpService {
  private readonly httpClient = inject(HttpClient);

  public loadOptions(): Observable<Configuration> {
    return this.httpClient.get<Configuration>('/api/configuration');
  }

  public uploadFile(file: File): Observable<UploadResponse> {
    const headers = new HttpHeaders()
      .append('Content-Type', file.type)
      .append('Content-Disposition', `filename="${file.name}"`)
    // .append('X-Forwarded-For', '127.0.0.1');

    return this.httpClient.post<UploadResponse>('/api/files', file.bytes, { headers });
  }
}