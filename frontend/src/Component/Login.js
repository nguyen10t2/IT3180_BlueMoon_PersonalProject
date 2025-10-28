import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import '../css/Login.css';

export default function Login() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState("");
  const navigate = useNavigate();

  const handleSubmit = async (e) => {
    e.preventDefault();

    if (!username.trim() || !password.trim()) {
      setMessage("Please fill in both fields!");
      return;
    }

    console.log("Sending login payload:", { username, password });

    try {
      const response = await fetch("http://127.0.0.1:8080/home/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({
          username: username.trim(),
          password_hash: password.trim()
        })
      });

      const text = await response.text();
      console.log("Server response:", response.status, text);

      if (!response.ok) {
        setMessage(text);
        return;
      }

      setMessage("Login successful!");


      navigate("/feedback");
    } catch (err) {
      console.error("Login fetch error:", err);
      setMessage("Network error or invalid credentials!");
    }
  };

  return (
    <div className="box">
      <div className="login-container">
        <h2>Login to Your Account</h2>
        <form className="Login" onSubmit={handleSubmit} noValidate>
          <label className="Account">
            <strong>Account</strong>
            <input
              type="text"
              placeholder="Username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
            />
          </label>
          <br />
          <label className="Password">
            <strong>Password</strong>
            <input
              type="password"
              placeholder="Password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
            />
          </label>
          <br />
          <button type="submit">Login</button>
        </form>
        <p className="title">
          Don't have an account? <a href="/register">Register</a>
        </p>
        <p className="message1">{message}</p>
      </div>
    </div>
  );
}
