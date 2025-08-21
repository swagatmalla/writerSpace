import {BrowserRouter, Routes, Route} from 'react-router-dom';
import AnalyzePage from "./pages/AnalyzePage";

export default function App(){
  return(
    <BrowserRouter>
      <Routes>
        <Route path = "/documents/:docId/analyze" element = {<AnalyzePage/>}/>
        <Route path = "/" element = {<div>Welcome to writerSpaceL Get an understanding of yourself!</div>}/>
      </Routes>
    </BrowserRouter>
  );
}