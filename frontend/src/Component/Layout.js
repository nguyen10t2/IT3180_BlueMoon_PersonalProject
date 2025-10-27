import React from "react";
import '../css/Layout.css';
import { useState } from "react"; 

export default function({sidebarGroups,children}){
    const [isClose, setIsclose]= useState("true");
    const [sidebar,  setSidebar] = useState(true); 
    const [name, setName] =useState('close');
   
    const handleSidebar = (e)=>{
        e.preventDefault();
        setSidebar(!sidebar);
        const aside =document.querySelector('aside');
        if(!sidebar) {
            setName('close');
        } else {
            setName('extend');
        }
    }
    const handleClose= (e)=>{
        e.preventDefault();
        const newState = !isClose;
        setIsclose(newState);
        const ul = e.target.nextElementSibling;
        const option =  newState ? 'closeOption' : 'extendOption';
        ul.setAttribute('class',option);
        
    }
    return(
        <div>
            <header className="header">
                <div>
                    <img src={sidebar ? '/apps.svg' : '/apps-add.svg' } className="tab" onClick={handleSidebar}></img>
                    <img src='/logo192.png' className="logo" ></img>
                    <h2 className="nameWeb">Proflow</h2>
                </div>
                <div>
                    <img src='/bell.svg' className="bell"></img>
                    <img src ='/user.svg' className="account"></img>
                </div>
            </header>
            <div className={`layout ${name}`}>
                <aside className={ `sidebar ${name}`}>
                    {sidebarGroups.map((group, index)=>(
                        <div key ={index}>
                            <h3 onClick={handleClose}>{group.title}</h3>
                            <ul >
                                {group.items.map((item,idx)=>(
                                    <li key ={idx}>{item}</li>
                                ))}
                            </ul>
                        </div>
                    ))}
                </aside>
                <section className="content">
                    <div className="info">
                        {children}
                    </div>
                </section>
            </div>
        </div>
    )
}