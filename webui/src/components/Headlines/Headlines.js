import React, { Component } from 'react';
import { Grid } from 'react-bootstrap';
import { Col } from 'react-bootstrap';
import { Row } from 'react-bootstrap';
import { PageHeader } from 'react-bootstrap';
import { ListGroup } from 'react-bootstrap';
import { ListGroupItem } from 'react-bootstrap';
import './Headlines.css';
import axios from 'axios';

const gridInstance = (
  <Grid>
    <Row className="show-grid">
      <Col xs={12} md={8}></Col>
      <Col xs={6} md={4}></Col>
    </Row>
  </Grid>
);




class Headlines extends Component {
  constructor(props) {
    super(props);

    this.state = {
      articles: [],
      num_articles: [0]
    };
  }
  componentDidMount() {
    axios.get(`feed/web.json`)
    .then(res => {
      const articles = res.data;
      this.setState({ articles });

      var total = 0;
      for (var a in articles) {
        total += parseInt(articles[a].numArticles);
      }
      const num_articles = [total];
      console.log(num_articles)
      this.setState({ num_articles });
    });
  }
  render() {
    return ( 
      <div>
        {this.state.num_articles.map(num =>
          <PageHeader><small>GE News from over</small> {num} <small>articles</small></PageHeader>
        )}

        <ListGroup>
          {this.state.articles.map(article =>
             <ListGroupItem header={article.titleText}>
              {article.mainText}
             </ListGroupItem>
          )}
        </ListGroup>
      </div>
    );
  }
}

export default Headlines;