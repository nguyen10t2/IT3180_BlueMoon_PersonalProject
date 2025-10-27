import logo from './logo.svg';
import './css/App.css';
import Login from './Component/Login.js';
import Register from './Component/Register.js';
import Admin from './Component/Admin.js'
import Citizens from './Component/Citizens.js';
import Feedback from './Component/Feedback.js';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Login />} />
        <Route path="/register" element={<Register />} />
        <Route path="/admin" element={<Admin />} />
        <Route path="/citizens" element={<Citizens />} />
        <Route path="/feedback" element={<Feedback />} />
      </Routes>
    </Router>
  );
}

export default App;
