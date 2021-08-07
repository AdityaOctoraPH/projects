import React, {useState, useEffect} from 'react';
import {Scatter} from 'react-chartjs-2';

const Graph=()=> {
  const[chartdata,setchartdata] = useState({});

  const chart=()=>{
    setchartdata({
      labels:'Data Penduduk',
      datasets:[
        {
          label:'Provinsi A',
          data:[{x:2,y:4},{x:1,y:4},{x:2,y:3},{x:5,y:4},{x:5,y:3}],
          backgroundColor:'#000000'
        },
        {
          label:'Provinsi B',
          data:[{x:3,y:4},{x:1,y:3},{x:5,y:3},{x:5,y:7},{x:5,y:1}],
          backgroundColor:'#FFCCCC'
        }
      ]
    })
  }

  useEffect(()=>{
    chart();
  },[]);

  return (
    <div>
      <Scatter 
        data={chartdata}
        height={300}
        width={600}
        options={{
          responsive:false,
          maintainAspectRatio:true
        }}
      />
    </div>
  )
}

export default Graph

