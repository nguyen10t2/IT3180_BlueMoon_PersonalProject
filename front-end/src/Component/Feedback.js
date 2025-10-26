import React ,{useState} from "react";
import { useNavigate } from "react-router-dom";
import './Layout'
import Layout from "./Layout";

export default function Feedback(){
    const groups =[
      {
        title:'Phản ánh',
        items: ['Tạo/Xóa phản ánh']
      },
      {
        title:'Thống kê',
        items:['Danh sách đã duyệt']
      }
    ]

    return (
    <div>
      <Layout sidebarGroups={groups}>


      </Layout>
    </div>
  );
}