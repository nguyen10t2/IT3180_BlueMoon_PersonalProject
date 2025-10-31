import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import '../css/Register.css'

export default function Citizens() {
    const [fullname, setFullname] = useState("");
    const [userName, setUserName] = useState("");
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [confirmPassword, setConfirmPassword] = useState("");
    const [error, setError] = useState("");
    const [error1, setError1] = useState("");
    const [role, setRole] = useState("")
    const navigate = useNavigate();
    const handleConfirm = (e) => {
        setConfirmPassword(e.target.value);
        if (password !== e.target.value) setError("Passwords do not match.");
        else setError("");
    }

    const handleSubmit = async (e) => {
        e.preventDefault();
        if (!userName.trim() || !password.trim() || !role.trim()
            || !confirmPassword.trim() || !email.trim() || !fullname.trim()) {
            setError1("Please fill in this field!")
            return;
        }
        try {
            const response = await fetch("http://127.0.0.1:8080/auth/register", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    username: userName,
                    fullname: fullname,
                    email: email,
                    password: password,
                    role: role
                })
            });
            if (!response.ok) {
                const mess = await response.text();
                setError1(mess);
                return;
            }
            setError1("User registered successfully!");
            navigate("/");
        }
        catch (err) {
            setError1("Registration failed. Try again.");
        }
    }
    return (
        <div className="box">
            <div className="register-container">
                <h2>Register</h2>
                <form className="Register" onSubmit={handleSubmit} >
                    <label className="fullname">
                        <strong>Full Name</strong>
                        <input type="text"
                            placeholder=""
                            value={fullname}
                            onChange={(e) => setFullname(e.target.value)}
                        />
                    </label>
                    <label className="userName">
                        <strong>Username</strong>
                        <input type="text"
                            placeholder=""
                            onChange={(e) => setUserName(e.target.value)}
                            value={userName}
                        />
                    </label>
                    <label className="Email">
                        <strong>Email</strong>
                        <input type="email"
                            placeholder=""
                            value={email}
                            onChange={(e) => setEmail(e.target.value)}
                        />
                    </label>
                    <label className="Password">
                        <strong>Password</strong>
                        <input type="password"
                            placeholder=""
                            value={password}
                            onChange={(e) => setPassword(e.target.value)}
                        />
                    </label>
                    <label className="ConfirmPassword">
                        <strong>Confirm Password</strong>
                        <input type="password"
                            placeholder=""
                            value={confirmPassword}
                            onChange={handleConfirm}
                        />
                        <p className="message">{error}</p>
                    </label>
                    <label className="Role">
                        <strong>Role</strong>
                        <input name="role" type="radio" value="admin"
                            onChange={(e) => setRole(e.target.value)}
                        />
                        <label>Admin Deverloper</label><br />
                        <input name="role" type="radio" value="manager"
                            onChange={(e) => setRole(e.target.value)}
                        />
                        <label>Ban quản lý</label><br />
                        <input name="role" type="radio" value="resident"
                            onChange={(e) => setRole(e.target.value)}
                        />
                        <label>Cư dân</label><br />
                    </label>
                    <button type="submit" >Register</button>
                </form>
                <p className="title">Yes i have an account? <a href="/">Login</a></p>
                <p className="message">{error1}</p>
            </div>
        </div>
    );
}