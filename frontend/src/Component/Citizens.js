import React ,{useState} from "react";
import { useNavigate } from "react-router-dom";
import './Layout'
import Layout from "./Layout";

export default function Citizens(){
    const groups =[
      {
        title:'Nhân khẩu',
        items: ['Đăng ký tạm vắng','Đăng ký tạm trú','Thêm nhân khẩu','Xóa nhân khẩu']
      },
      {
        title:'Hộ khẩu',
        items: ['Tách hộ khẩu', 'Nhập hộ khẩu','Thêm hộ khẩu','Xóa hộ khẩu']
      },
      {
        title:'Thống kê',
        items:['Nhân khẩu', 'Hộ khẩu']
      }
    ]

    return (
    <div>
      <Layout sidebarGroups={groups}>


      </Layout>
    </div>
  );
}