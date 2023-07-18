const baseUrl = "https://rel-kurzumfrage-service.alex-dev.net";
// const baseUrl = "http://localhost:8080";

export async function post<T>(path: string, body: any) {
  return await fetch(baseUrl + path, {
    method: "POST",
    body: JSON.stringify(body),
    headers: {
      "Content-Type": "application/json",
    },
  }).then((it) => it.json() as T);
}

export async function get<T>(path: string) {
  return await fetch(baseUrl + path, {
    method: "GET",
  }).then((it) => it.json() as T);
}

