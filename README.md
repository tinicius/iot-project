<a name="readme-top"></a>

<!-- PROJECT SHIELDS -->

<p align="center">
  <a href="https://skillicons.dev">
    <img src="https://skillicons.dev/icons?i=aws,git,nodejs,rabbitmq,rust,docker" />
  </a>
</p>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  
  <h1 align="center">Iot Project</h1>

  

  <p align="center">
    An backend project focusing on cutting-edge IoT services and patterns
  </p>
</div>

[![en](https://img.shields.io/badge/lang-en-red.svg)](https://github.com/tinicius/iot-project/blob/main/README.md)
[![pt-br](https://img.shields.io/badge/lang-pt--br-green.svg)](https://github.com/tinicius/iot-project/blob/main/README.pt-br.md)

<!-- TABLE OF CONTENTS -->

  <summary>Table of Contents</summary>  
  <ol>
    <li>
      <a href="#about-the-project">Introduction</a>
    <li>
      <a href="#getting started">Getting started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
      </ul>
      <ul>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li>
      <a href="#services">Services</a>
      <ul>
        <li><a href="#device-simulator">Device Simulator</a></li>
        <li><a href="#consumer">Consumer</a></li>
        <li><a href="#historian">Historian</a></li>
        <li><a href="#api">Api</a></li>
      </ul>
    </li>
    <li>
      <a href="#tech">Tech</a>
      <ul>
        <li><a href="#mqtt">MQTT</a></li>
        <li><a href="#rabbitmq">RabbitMQ</a></li>
        <li><a href="#timeseries">Timeseries (AWS Timestream)</a></li>
        <li><a href="#grpc">gRPC</a></li>
      </ul>
    </li>
    <li>
      <a href="#progress">In progress</a>
      <ul>
        <li><a href="#apihttp">API HTTP</a></li>
        <li><a href="#kafka">Kafka</a></li>
        <li><a href="#websocket">WebSocket</a></li>
        <li><a href="#front">Front end (?)</a></li>
      </ul>
    </li>
    <li><a href="#conclusion">Conclusion</a></li>
  </ol>

<!-- ABOUT THE PROJECT -->
## About The Project

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

Our application is distributed via Docker containers, making it easy to install and run in various environments.

### Prerequisites

Make sure you have Docker installed on your machine. If you don't have it yet, you can download and install it from the [official Docker website](docker.com).

Além disso, é necessário configurar algumas variáveis de ambiente que serão utilizadas pelo programa.

```
export AWS_ACCESS_KEY_ID=<your_acess_key_id>
export AWS_SECRET_ACCESS_KEY=<yout_secret_acess_key>
export AWS_DEFAULT_REGION=<your_default_region>
```

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/tinicius/iot-project.git
   ```
2. Run docker compose
   ```sh
   docker compose -f "docker-compose.yml" up -d --build 
   ```


<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

Para validar todo o fluxo em funcionamento é possivel acessar alguns endpoints que são expostos pela aplicação.

### MQTT

O topico raiz usado pelo projeto é *IoTProject/#*

```
HOST: localhost
PORT: 1883
USER: admin
PASSWORD: admin
```

### RabbitMQ

No RabbitMQ temos a exchange principal *IOT_PROJECT*

E também duas filas *TEMP* e *HUMIDITY*

```
HOST=localhost
PORT=15672
USER=guest
PASSWORD=guest
```

### API gRPC

Para acessar o api você irá precisar do arquivo [server.proto](https://github.com/tinicius/iot-project/blob/9d1da11972804a3c261826e70fd79679c12f528d/api/protos/server.proto)

O servidor está disponivel na rota *0.0.0.0:50051*

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the MIT License.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->
## Contact

Vinicius Alves Pereira


[![text](https://img.shields.io/badge/LinkedIn-0077B5?style=for-the-badge&logo=linkedin&logoColor=white)](www.linkedin.com/in/dev-vini-pereira)


<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/othneildrew/Best-README-Template.svg?style=for-the-badge
[contributors-url]: https://github.com/othneildrew/Best-README-Template/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/othneildrew/Best-README-Template.svg?style=for-the-badge
[forks-url]: https://github.com/othneildrew/Best-README-Template/network/members
[stars-shield]: https://img.shields.io/github/stars/othneildrew/Best-README-Template.svg?style=for-the-badge
[stars-url]: https://github.com/othneildrew/Best-README-Template/stargazers
[issues-shield]: https://img.shields.io/github/issues/othneildrew/Best-README-Template.svg?style=for-the-badge
[issues-url]: https://github.com/othneildrew/Best-README-Template/issues
[license-shield]: https://img.shields.io/github/license/othneildrew/Best-README-Template.svg?style=for-the-badge
[license-url]: https://github.com/othneildrew/Best-README-Template/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/othneildrew
[product-screenshot]: images/screenshot.png
[Next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Next-url]: https://nextjs.org/
[React.js]: https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB
[React-url]: https://reactjs.org/
[Vue.js]: https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D
[Vue-url]: https://vuejs.org/
[Angular.io]: https://img.shields.io/badge/Angular-DD0031?style=for-the-badge&logo=angular&logoColor=white
[Angular-url]: https://angular.io/
[Svelte.dev]: https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00
[Svelte-url]: https://svelte.dev/
[Laravel.com]: https://img.shields.io/badge/Laravel-FF2D20?style=for-the-badge&logo=laravel&logoColor=white
[Laravel-url]: https://laravel.com
[Bootstrap.com]: https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white
[Bootstrap-url]: https://getbootstrap.com
[JQuery.com]: https://img.shields.io/badge/jQuery-0769AD?style=for-the-badge&logo=jquery&logoColor=white
[JQuery-url]: https://jquery.com
