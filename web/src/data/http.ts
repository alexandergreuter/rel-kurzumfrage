export async function post<T>(path: string, body: any) {
  return await fetch("http://localhost:8080" + path, {
    method: "POST",
    body: JSON.stringify(body),
    headers: {
      "Content-Type": "application/json",
    },
  }).then((it) => it.json() as T);
}

export async function get<T>(path: string) {
  return await fetch("http://localhost:8080" + path, {
    method: "GET",
  }).then((it) => it.json() as T);
}

