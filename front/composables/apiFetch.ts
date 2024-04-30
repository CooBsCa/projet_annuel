import { useAuthStore } from "~/stores/auth";
export const apiFetch = async (request: any, opts?: any) => {
  if (!opts) opts = {};
  const authStore = useAuthStore();
  let token = authStore.getToken();

  if (token == "" || token == null) navigateTo("/home");

  const config = useRuntimeConfig();

  opts.headers = {
    ...opts.headers,
    accept: "application/json",
    "Content-Type": "application/json",
    Authorization: `Bearer ${token}`,
  };

  try {
    return await fetch(config.public.baseURL + request, opts);
  } catch (error: any) {
    if (error.response && error.response.status === 401) {
      navigateTo("/home");
    }
    throw error;
  }
  return await fetch(config.public.baseURL + request, opts);
};

export const apiGet = async (request: any, opts?: any) => {
  if (!opts) opts = {};
  return await apiFetch(request, { method: "GET", ...opts });
};

export const apiPost = async (request: any, opts?: any) => {
  if (!opts) opts = {};
  return await apiFetch(request, { method: "POST", ...opts });
};

export const apiPut = async (request: any, opts?: any) => {
  if (!opts) opts = {};
  return await apiFetch(request, { method: "PUT", ...opts });
};

export const apiDelete = async (request: any, opts?: any) => {
  if (!opts) opts = {};
  return await apiFetch(request, { method: "DELETE", ...opts });
};

export const apiPatch = async (request: any, opts?: any) => {
  if (!opts) opts = {};
  return await apiFetch(request, { method: "PATCH", ...opts });
};
