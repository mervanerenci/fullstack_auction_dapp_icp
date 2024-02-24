import React, { Component } from 'react';

import './Navbar.scss';

export default class Navbar extends Component {
  state = {
    menus: ['New Auction', 'Auctions', 'Media', 'Meet us']
  }
  render() {
    
    const menuItem = this.state.menus.map(menu => <li key={menu} className="menu_item">{menu}</li> );

    return (
      <nav className="nav">
        <label htmlFor="toggle"><img src="" alt=""/></label>
        <input type="checkbox" id="toggle" />
        <ul className="menu">
          {menuItem}
        </ul>
      </nav>
    )
  }
}