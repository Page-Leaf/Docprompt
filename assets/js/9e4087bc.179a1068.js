"use strict";(self.webpackChunkdocprompt=self.webpackChunkdocprompt||[]).push([[711],{9331:(e,r,t)=>{t.r(r),t.d(r,{default:()=>o});t(6540);var s=t(8774),a=t(1312),i=t(1003),c=t(781),n=t(1107),l=t(4848);function d(e){let{year:r,posts:t}=e;return(0,l.jsxs)(l.Fragment,{children:[(0,l.jsx)(n.A,{as:"h3",id:r,children:r}),(0,l.jsx)("ul",{children:t.map((e=>(0,l.jsx)("li",{children:(0,l.jsxs)(s.A,{to:e.metadata.permalink,children:[e.metadata.formattedDate," - ",e.metadata.title]})},e.metadata.date)))})]})}function h(e){let{years:r}=e;return(0,l.jsx)("section",{className:"margin-vert--lg",children:(0,l.jsx)("div",{className:"container",children:(0,l.jsx)("div",{className:"row",children:r.map(((e,r)=>(0,l.jsx)("div",{className:"col col--4 margin-vert--lg",children:(0,l.jsx)(d,{...e})},r)))})})})}function o(e){let{archive:r}=e;const t=(0,a.T)({id:"theme.blog.archive.title",message:"Archive",description:"The page & hero title of the blog archive page"}),s=(0,a.T)({id:"theme.blog.archive.description",message:"Archive",description:"The page & hero description of the blog archive page"}),d=function(e){const r=e.reduce(((e,r)=>{const t=r.metadata.date.split("-")[0],s=e.get(t)??[];return e.set(t,[r,...s])}),new Map);return Array.from(r,(e=>{let[r,t]=e;return{year:r,posts:t}}))}(r.blogPosts);return(0,l.jsxs)(l.Fragment,{children:[(0,l.jsx)(i.be,{title:t,description:s}),(0,l.jsxs)(c.A,{children:[(0,l.jsx)("header",{className:"hero hero--primary",children:(0,l.jsxs)("div",{className:"container",children:[(0,l.jsx)(n.A,{as:"h1",className:"hero__title",children:t}),(0,l.jsx)("p",{className:"hero__subtitle",children:s})]})}),(0,l.jsx)("main",{children:d.length>0&&(0,l.jsx)(h,{years:d})})]})]})}}}]);