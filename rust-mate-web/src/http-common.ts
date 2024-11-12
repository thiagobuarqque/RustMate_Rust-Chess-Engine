import axios from "axios";

export const https = axios.create({
  baseURL: `http://${window.location.hostname}:8000`,
  headers: {
    "Content-type": "application/json"
  }
});

export function postWithBody<T = any>(
  endpoint: string,
  body: any,
  onSuccess: (data: T) => void,
  onError?: (reason: any) => void): void{

    https.post<T>(endpoint, body)
      .then(response => onSuccess(response.data))
      .catch(onError)
}

export function get<T = any>(
  endpoint: string,
  onSuccess: (data: T) => void,
  onError?: (reason: any) => void): void{

    https.get<T>(endpoint)
      .then(response => onSuccess(response.data))
      .catch(onError)
}

export default https;