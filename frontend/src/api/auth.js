export async function refreshAccessToken() {
    const refresh_token = localStorage.getItem("refresh_token");
    if (!refresh_token) return null;

    try {
        const res = await fetch("http://127.0.0.1:8080/auth/refresh", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ refresh_token }),
        });

        if (!res.ok) throw new Error("Invalid refresh token");

        const data = await res.json();
        localStorage.setItem("access_token", data.token);
        return data.token;
    } catch (err) {
        console.error("Failed to refresh token:", err);
        return null;
    }
}

export async function fetchWithAuth(url, options = {}) {
    let token = localStorage.getItem("access_token");

    let res = await fetch(url, {
        ...options,
        headers: {
            ...options.headers,
            Authorization: `Bearer ${token}`,
        },
    });
    
    if (res.status === 401) {
        token = await refreshAccessToken();
        if (!token) throw new Error("Session expired, please login again");

        res = await fetch(url, {
            ...options,
            headers: {
                ...options.headers,
                Authorization: `Bearer ${token}`,
            },
        });
    }

    return res;
}
