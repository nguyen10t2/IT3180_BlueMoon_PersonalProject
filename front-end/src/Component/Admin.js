import React ,{useState} from "react";
import { useNavigate } from "react-router-dom";
import './Layout'
import Layout from "./Layout";

export default function Admin(){
    const groups =[
      {
        title:'Nhân khẩu',
        items: ['Xử lý yêu cầu']
      },
      {
        title:'Hộ khẩu',
        items:['Xử lý yêu cầu']
      },
      {
        title:'Phản ánh',
        items:['Xử lý yêu cầu']
      },
      {
        title:'Thống kê',
        items:['Nhân khẩu','Hộ khẩu','Phản ánh']
      }
    ]

    return (
    <div>
      <Layout sidebarGroups={groups}>


      </Layout>
    </div>
  );
}